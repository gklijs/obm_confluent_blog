package nl.openweb.commandhandler

import nl.openweb.commandhandler.Utils.invalidFrom
import nl.openweb.commandhandler.Utils.isValidOpenIban
import nl.openweb.data.BalanceChanged
import nl.openweb.data.ConfirmMoneyTransfer
import nl.openweb.data.MoneyTransferConfirmed
import nl.openweb.data.MoneyTransferFailed
import org.apache.avro.specific.SpecificRecord
import org.apache.kafka.streams.KeyValue
import org.apache.kafka.streams.kstream.KStream
import org.apache.kafka.streams.kstream.Predicate
import org.springframework.boot.autoconfigure.EnableAutoConfiguration
import org.springframework.cloud.stream.annotation.EnableBinding
import org.springframework.cloud.stream.annotation.StreamListener
import org.springframework.data.repository.findByIdOrNull
import org.springframework.messaging.handler.annotation.SendTo
import java.time.LocalDateTime
import java.util.*

@EnableBinding(MoneyTransfer::class)
@EnableAutoConfiguration
class MoneyTransferProcessor(private val balanceRepository: BalanceRepository, private val cmtRepository: CmtRepository) {

    @StreamListener(MoneyTransfer.CMT)
    @SendTo(MoneyTransfer.MTF, MoneyTransfer.BACH)
    fun on(input: KStream<String, ConfirmMoneyTransfer>): Array<out KStream<String, SpecificRecord>>? {
        val isFeedback = Predicate { _: String, v: SpecificRecord -> v is MoneyTransferConfirmed || v is MoneyTransferFailed }
        val isBalance = Predicate { _: String, v: SpecificRecord -> v is BalanceChanged }

        return input
                .flatMap(this::getResponses)
                .branch(isFeedback, isBalance)
    }

    private fun getResponses(k: String, v: ConfirmMoneyTransfer): List<KeyValue<String, SpecificRecord>> {
        return cmtRepository.findByIdOrNull(UUID.fromString(k))?.let { cmtHandled(k, v, it) } ?: handleMoneyTransfer(k, v)
    }

    private fun cmtHandled(k: String, v: ConfirmMoneyTransfer, cmt: Cmt): List<KeyValue<String, SpecificRecord>> {
        val result: SpecificRecord = if (cmt.reason.isBlank()) {
            MoneyTransferConfirmed(v.id)
        } else {
            MoneyTransferFailed(v.id, cmt.reason)
        }
        return listOf(KeyValue(k, result))
    }

    private fun handleMoneyTransfer(k: String, v: ConfirmMoneyTransfer): List<KeyValue<String, SpecificRecord>> {
        val reason: String
        lateinit var transfer: Result
        when {
            v.from.invalidFrom() -> reason = "from is invalid"
            v.from == v.to -> reason = "from and to can't be same for transfer"
            else -> {
                transfer = tryTransfer(k, v)
                reason = transfer.reason
            }
        }
        cmtRepository.save(Cmt(UUID.fromString(k), reason, LocalDateTime.now()))
        return if (reason.isBlank()) {
            transfer.responses
        } else {
            listOf(KeyValue(k, MoneyTransferFailed(v.id, reason) as SpecificRecord))
        }
    }

    private fun tryTransfer(k: String, v: ConfirmMoneyTransfer): Result {
        val responses = mutableListOf<KeyValue<String, SpecificRecord>>()
        val fromList = if (v.from.isValidOpenIban()) balanceRepository.findByIban(v.from) else Collections.emptyList()
        val now = LocalDateTime.now()
        if (fromList.isNotEmpty()) {
            val from = fromList.first()
            when {
                v.token != from.token -> return Result("invalid token", responses)
                from.amount - v.amount < from.lmt -> return Result("insufficient funds", responses)
                else -> {
                    val newFrom = Balance(from.balanceId, from.iban, from.token, from.amount - v.amount, from.type, from.lmt, from.createdAt, now)
                    balanceRepository.save(newFrom)
                    responses.add(KeyValue(from.iban, BalanceChanged(from.iban, newFrom.amount, -v.amount, v.to, v.description)))
                }
            }
        }
        val toList = if (v.to.isValidOpenIban()) balanceRepository.findByIban(v.to) else Collections.emptyList()
        if (toList.isNotEmpty()) {
            val to = toList.first()
            val newTo = Balance(to.balanceId, to.iban, to.token, to.amount + v.amount, to.type, to.lmt, to.createdAt, now)
            balanceRepository.save(newTo)
            responses.add(KeyValue(to.iban, BalanceChanged(to.iban, newTo.amount, v.amount, v.from, v.description)))
        }
        responses.add(KeyValue(k, MoneyTransferConfirmed(v.id)))
        return Result("", responses)
    }

    data class Result(val reason: String, val responses: List<KeyValue<String, SpecificRecord>>)
}
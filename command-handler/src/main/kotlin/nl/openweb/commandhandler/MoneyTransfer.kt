package nl.openweb.commandhandler

import nl.openweb.data.BalanceChanged
import nl.openweb.data.ConfirmMoneyTransfer
import org.apache.avro.specific.SpecificRecord
import org.apache.kafka.streams.kstream.KStream
import org.springframework.cloud.stream.annotation.Input
import org.springframework.cloud.stream.annotation.Output

interface MoneyTransfer {
    @Input(CMT)
    fun input(): KStream<String, ConfirmMoneyTransfer>

    @Output(MTF)
    fun feedback(): KStream<String, SpecificRecord>

    @Output(BACH)
    fun balance(): KStream<String, BalanceChanged>

    companion object {
        const val CMT = "cmt"
        const val MTF = "mtf"
        const val BACH = "bach"
    }
}
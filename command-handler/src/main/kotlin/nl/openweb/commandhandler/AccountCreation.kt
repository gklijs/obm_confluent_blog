package nl.openweb.commandhandler

import nl.openweb.data.ConfirmAccountCreation
import org.apache.avro.specific.SpecificRecord
import org.apache.kafka.streams.kstream.KStream
import org.springframework.cloud.stream.annotation.Input
import org.springframework.cloud.stream.annotation.Output

interface AccountCreation {
    @Input(CAC)
    fun input(): KStream<String, ConfirmAccountCreation>

    @Output(ACF)
    fun feedback(): KStream<String, SpecificRecord>

    companion object {
        const val CAC = "cac"
        const val ACF = "acf"
    }
}
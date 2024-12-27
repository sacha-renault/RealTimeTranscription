import { createModel, Model, KaldiRecognizer } from "vosk-browser";


export interface VoskRecognitionObject {
    model: Model,
    recognizer: KaldiRecognizer;
}

interface InnerResult {
    result: Array<object>;
    text: string;
}

interface InnerPartialResult {
    partial: string;
}

export interface VoskPartialResult {
    event: string;
    recognizerId: string;
    result: InnerPartialResult;
}

export interface VoskResult {
    event: string;
    recognizerId: string;
    result: InnerResult;
}

async function requestMicrophoneAccess(): Promise<MediaStream> {
    try {
        const stream = await navigator.mediaDevices.getUserMedia({
            video: false,
            audio: {
                echoCancellation: true,
                noiseSuppression: true,
                channelCount: 1,
                sampleRate: 16000
            },
        });
        return stream;
    } catch (err) {
        throw err;
    }
}

export async function initialize(modelPath: string): Promise<VoskRecognitionObject> {
    // Create a model
    const model = await createModel(modelPath);

    // recognizer
    const recognizer = new model.KaldiRecognizer(48000);
    recognizer.setWords(true);

    // Ask user for audio
    const mediaStream = await requestMicrophoneAccess();

    const audioContext = new AudioContext();
    const recognizerNode = audioContext.createScriptProcessor(4096, 1, 1)
    recognizerNode.onaudioprocess = (event) => {
        try {
            recognizer.acceptWaveform(event.inputBuffer)
        } catch (error) {
            throw error;
        }
    }
    const source = audioContext.createMediaStreamSource(mediaStream);
    source.connect(recognizerNode);
    recognizerNode.connect(audioContext.destination);
    return { model: model, recognizer: recognizer };
}
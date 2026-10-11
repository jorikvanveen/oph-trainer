use super::{AnswerCriteria, QuestionBank};
use crate::dto::{QuestionDto, TopicDto};

const AUDIO_FORMATS_SOURCE: &str = "https://interview.orpheus.network/audio-formats.php";
const MP3_SOURCE: &str = "https://interview.orpheus.network/mp3.php";
const TRANSCODES_SOURCE: &str = "https://interview.orpheus.network/transcodes.php";

pub(super) fn bank() -> QuestionBank {
    QuestionBank {
        topics: vec![
            TopicDto {
                topic_id: "formats".into(),
                name: "Audio formats".into(),
            },
            TopicDto {
                topic_id: "mp3".into(),
                name: "MP3".into(),
            },
            TopicDto {
                topic_id: "transcodes".into(),
                name: "Transcodes".into(),
            },
        ],
        questions: vec![
            QuestionDto {
                question_id: "formats-flac".into(),
                topic_id: "formats".into(),
                material: vec![],
                prompt: "What does FLAC stand for?".into(),
                reference_answer: "Free Lossless Audio Codec, a compressed lossless format.".into(),
                source_url: AUDIO_FORMATS_SOURCE.into(),
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["Free Lossless Audio Codec".into()],
                    aliases: vec![],
                },
            },
            QuestionDto {
                question_id: "formats-allowed-lossless".into(),
                topic_id: "formats".into(),
                material: vec![],
                prompt: "Which lossless audio format is allowed for uploads on Orpheus?".into(),
                reference_answer: "FLAC is the only allowed lossless upload format.".into(),
                source_url: AUDIO_FORMATS_SOURCE.into(),
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["FLAC".into()],
                    aliases: vec!["Free Lossless Audio Codec".into()],
                },
            },
            QuestionDto {
                question_id: "formats-lossless".into(),
                topic_id: "formats".into(),
                material: vec![],
                prompt: "How can compressed lossless audio use less space without losing audio information?".into(),
                reference_answer: "It represents the same audio more compactly, for example using very few bits for silence. Decoding reproduces all the recorded audio data.".into(),
                source_url: AUDIO_FORMATS_SOURCE.into(),
                grading: AnswerCriteria::Explanation {
                    required_points: vec!["Compression uses a more efficient encoding, such as representing silence with very few bits instead of the fixed uncompressed allocation.".into()],
                    optional_points: vec![],
                    contradictions: vec!["Compression removes audio information or inaudible frequencies.".into(), "Lossless compression reduces audio quality.".into()],
                },
            },
            QuestionDto {
                question_id: "formats-transparency".into(),
                topic_id: "formats".into(),
                material: vec![],
                prompt: "What does it mean for a lossy audio file to be transparent?".into(),
                reference_answer: "An average listener cannot hear a difference from the same song in lossless form when listening without knowing which version is playing.".into(),
                source_url: AUDIO_FORMATS_SOURCE.into(),
                grading: AnswerCriteria::Explanation {
                    required_points: vec!["An average listener cannot distinguish the lossy encode from the same recording in lossless form in a blind listening comparison.".into()],
                    optional_points: vec![],
                    contradictions: vec!["A transparent lossy file retains all original audio data.".into(), "Transparency guarantees that every listener hears no difference.".into()],
                },
            },
            QuestionDto {
                question_id: "mp3-vbr".into(),
                topic_id: "mp3".into(),
                material: vec![],
                prompt: "What does VBR stand for?".into(),
                reference_answer: "Variable bitrate.".into(),
                source_url: MP3_SOURCE.into(),
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["Variable bitrate".into()],
                    aliases: vec!["Variable bit rate".into()],
                },
            },
            QuestionDto {
                question_id: "mp3-lame".into(),
                topic_id: "mp3".into(),
                material: vec![],
                prompt: "Which MP3 encoder is recommended for uploads to Orpheus?".into(),
                reference_answer: "LAME, for its open source code, customization, and high quality output.".into(),
                source_url: MP3_SOURCE.into(),
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["LAME".into()],
                    aliases: vec!["LAME encoder".into(), "The LAME encoder".into()],
                },
            },
            QuestionDto {
                question_id: "mp3-cbr".into(),
                topic_id: "mp3".into(),
                material: vec![],
                prompt: "How does CBR encoding allocate bits to silence and complex audio, and what does that mean for file size?".into(),
                reference_answer: "CBR maintains a selected bitrate. Silence and complex sound receive the same bits per second, making file size predictable.".into(),
                source_url: MP3_SOURCE.into(),
                grading: AnswerCriteria::Explanation {
                    required_points: vec!["Silence and complex audio receive the same chosen number of bits per second.".into(), "File size is predictable.".into()],
                    optional_points: vec!["CBR files are larger than VBR or ABR files at the same quality.".into()],
                    contradictions: vec!["CBR varies bitrate with audio complexity.".into(), "CBR allocates fewer bits per second to silence.".into()],
                },
            },
            QuestionDto {
                question_id: "mp3-vbr-quality".into(),
                topic_id: "mp3".into(),
                material: vec![],
                prompt: "What does VBR encoding target, and how does audio complexity affect its bitrate?".into(),
                reference_answer: "VBR targets a chosen quality. Bitrate varies with complexity, using fewer bits for silence and more for complex sound.".into(),
                source_url: MP3_SOURCE.into(),
                grading: AnswerCriteria::Explanation {
                    required_points: vec!["The encoder targets a selected quality.".into(), "Bitrate varies with complexity, using fewer bits for simpler audio and more for complex audio.".into()],
                    optional_points: vec!["Exact file size is less predictable than with CBR.".into()],
                    contradictions: vec!["VBR maintains one constant bitrate throughout the file.".into(), "VBR guarantees a preset average bitrate.".into()],
                },
            },
            QuestionDto {
                question_id: "transcodes-definition".into(),
                topic_id: "transcodes".into(),
                material: vec![],
                prompt: "What is the process of converting an audio file from one format to another called?".into(),
                reference_answer: "Transcoding. The resulting converted file is a transcode.".into(),
                source_url: TRANSCODES_SOURCE.into(),
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["Transcoding".into()],
                    aliases: vec!["Transcode".into(), "A transcode".into()],
                },
            },
            QuestionDto {
                question_id: "transcodes-lossy-source".into(),
                topic_id: "transcodes".into(),
                material: vec![],
                prompt: "Is re-encoding an MP3 into another MP3 a good or bad transcode under Orpheus rules?".into(),
                reference_answer: "Bad. Any transcode from a lossy source is prohibited.".into(),
                source_url: TRANSCODES_SOURCE.into(),
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["Bad".into()],
                    aliases: vec!["Bad transcode".into(), "A bad transcode".into()],
                },
            },
            QuestionDto {
                question_id: "transcodes-lossy-flac".into(),
                topic_id: "transcodes".into(),
                material: vec![],
                prompt: "An MP3 file is converted to FLAC. How does this affect the audio data and quality?".into(),
                reference_answer: "MP3 has already discarded audio data. FLAC cannot restore it. This lossy-to-lossless transcode is prohibited on Orpheus.".into(),
                source_url: TRANSCODES_SOURCE.into(),
                grading: AnswerCriteria::Explanation {
                    required_points: vec!["The MP3 source already lost audio data that FLAC cannot restore.".into()],
                    optional_points: vec!["Lossy-to-lossless conversion is a bad transcode prohibited on Orpheus.".into()],
                    contradictions: vec!["FLAC restores audio information lost during MP3 encoding.".into(), "A lossless output format makes a lossy source an acceptable lossless original.".into()],
                },
            },
            QuestionDto {
                question_id: "transcodes-lossless".into(),
                topic_id: "transcodes".into(),
                material: vec![],
                prompt: "Is converting a lossless WAV file to FLAC a good transcode? Explain why.".into(),
                reference_answer: "Yes. Lossless-to-lossless conversion preserves the original audio data and quality.".into(),
                source_url: TRANSCODES_SOURCE.into(),
                grading: AnswerCriteria::Explanation {
                    required_points: vec!["Yes, it is a good transcode because the original audio data and quality are preserved.".into()],
                    optional_points: vec!["FLAC compresses the audio to reduce storage.".into()],
                    contradictions: vec!["Converting lossless WAV to FLAC discards audio data.".into(), "Every transcode is bad regardless of its source.".into()],
                },
            },
        ],
    }
}

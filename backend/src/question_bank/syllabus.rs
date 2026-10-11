use super::{AnswerCriteria, QuestionBank};
use crate::dto::{ExerciseMaterialDto, QuestionDto, TopicDto};

const SOURCES: &str = "https://interview.orpheus.network/analog-and-digital-music-sources.php";
const FORMATS: &str = "https://interview.orpheus.network/audio-formats.php";
const MP3: &str = "https://interview.orpheus.network/mp3.php";
const TRANSCODES: &str = "https://interview.orpheus.network/transcodes.php";
const TORRENTING: &str = "https://interview.orpheus.network/torrenting.php";
const SPECTRALS: &str = "https://interview.orpheus.network/spectral-analysis.php";
const CD: &str = "https://interview.orpheus.network/cd-burning-and-cd-ripping.php";
const DUPES: &str = "https://interview.orpheus.network/dupes-and-trumps.php";
const RULES: &str = "https://interview.orpheus.network/orpheus-rules.php";

pub(super) fn bank() -> QuestionBank {
    QuestionBank {
        topics: vec![
            TopicDto {
                topic_id: "sources".into(),
                name: "Analog and digital music sources".into(),
            },
            TopicDto {
                topic_id: "torrenting".into(),
                name: "Torrenting".into(),
            },
            TopicDto {
                topic_id: "spectral".into(),
                name: "Spectral analysis".into(),
            },
            TopicDto {
                topic_id: "cd".into(),
                name: "CD burning and ripping".into(),
            },
            TopicDto {
                topic_id: "dupes".into(),
                name: "Dupes and trumps".into(),
            },
            TopicDto {
                topic_id: "rules".into(),
                name: "Orpheus rules".into(),
            },
        ],
        questions: vec![
            QuestionDto {
                question_id: "sources-converter".into(),
                topic_id: "sources".into(),
                prompt: "What device converts an analog recording into a digital file?".into(),
                reference_answer: "An analog-to-digital converter, such as a sound card.".into(),
                source_url: SOURCES.into(),
                material: vec![],
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["Analog-to-digital converter".into()],
                    aliases: vec![
                        "ADC".into(),
                        "Analog to digital converter".into(),
                        "A sound card".into(),
                        "Sound card".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "sources-media".into(),
                topic_id: "sources".into(),
                prompt: "How do cassette tape and vinyl store sound?".into(),
                reference_answer: "Cassette tape stores changes in magnetization; vinyl stores physical grooves representing sound.".into(),
                source_url: SOURCES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Tape stores changes in magnetization.".into(),
                        "Vinyl stores physical grooves.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "Vinyl already stores computer-readable digital files.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "sources-digitizing".into(),
                topic_id: "sources".into(),
                prompt: "How are cassette tape and vinyl recordings copied to digital files?".into(),
                reference_answer: "Play the medium into a recording device or sound card that converts analog audio to digital data. Vinyl playback uses a preamp before the sound card.".into(),
                source_url: SOURCES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Playback is captured through an analog-to-digital recording device.".into(),
                        "Vinyl playback uses a preamp.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "sources-approval".into(),
                topic_id: "sources".into(),
                prompt: "Which analog music sources need moderator approval before upload to Orpheus, and which source is always approved?".into(),
                reference_answer: "Vinyl is approved. Cassette tapes and other analog sources require moderator approval.".into(),
                source_url: SOURCES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Vinyl is approved.".into(),
                        "Cassette and other analog media require moderator approval.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "All analog sources can be uploaded without approval.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "sources-digital".into(),
                topic_id: "sources".into(),
                prompt: "What distinguishes a digital music source? Give two examples.".into(),
                reference_answer: "Digital sources already store computer-readable binary data. Examples include CDs, DVDs, SACDs and WEB downloads.".into(),
                source_url: SOURCES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Digital media already store computer-readable data.".into(),
                        "Give two of CD, DVD, SACD or WEB download.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "sources-digital-check".into(),
                topic_id: "sources".into(),
                prompt: "What check is needed before uploading music from a digital source to Orpheus?".into(),
                reference_answer: "Use spectral analysis to check for lossy transcodes before uploading.".into(),
                source_url: SOURCES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Check for lossy transcodes with spectral analysis.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "A digital source guarantees that its audio is lossless.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "formats-categories".into(),
                topic_id: "formats".into(),
                prompt: "Group AAC, FLAC, WAV, MP3, ALAC, DTS, AIFF, WMA, APE and AC3 by compression type, and name each group.".into(),
                reference_answer: "WAV and AIFF are uncompressed lossless PCM. FLAC, ALAC and APE are compressed lossless. MP3, AAC, WMA, AC3 and DTS are lossy.".into(),
                source_url: FORMATS.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "WAV is uncompressed lossless.".into(),
                        "AIFF is uncompressed lossless.".into(),
                        "FLAC is compressed lossless.".into(),
                        "ALAC is compressed lossless.".into(),
                        "APE is compressed lossless.".into(),
                        "MP3 is lossy.".into(),
                        "AAC is lossy.".into(),
                        "WMA is lossy.".into(),
                        "AC3 is lossy.".into(),
                        "DTS is lossy.".into(),
                    ],
                    optional_points: vec!["PCM means pulse-code modulation.".into()],
                    contradictions: vec!["FLAC is lossy.".into()],
                },
            },
            QuestionDto {
                question_id: "formats-bitrate-unit".into(),
                topic_id: "formats".into(),
                prompt: "What unit is commonly used to express audio bitrate?".into(),
                reference_answer: "Kilobits per second, abbreviated kbps.".into(),
                source_url: FORMATS.into(),
                material: vec![],
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["Kilobits per second".into()],
                    aliases: vec!["kbps".into(), "kbit/s".into()],
                },
            },
            QuestionDto {
                question_id: "formats-bitrate-definition".into(),
                topic_id: "formats".into(),
                prompt: "What does audio bitrate measure, and what does 320 kbps mean?".into(),
                reference_answer: "Bitrate measures the number of bits conveyed per unit of time. At 320 kbps, 320 kilobits are conveyed per second.".into(),
                source_url: FORMATS.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Bitrate measures bits conveyed per unit of time, so 320 kbps means 320 kilobits per second.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "Bitrate measures audio frequency or song duration.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "formats-size".into(),
                topic_id: "formats".into(),
                prompt: "For the same recording, why do uncompressed lossless, compressed lossless and lossy files use different amounts of space?".into(),
                reference_answer: "Uncompressed lossless gives silence the same bits per second as sound. Compressed lossless encodes the audio more compactly, using very few bits for silence. Lossy encoding saves space by discarding some original audio data.".into(),
                source_url: FORMATS.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Uncompressed lossless uses the same bits per second for silence and sound.".into(),
                        "Compressed lossless represents the same audio more compactly, such as using very few bits for silence.".into(),
                        "Lossy encoding saves space by discarding part of the recorded data.".into(),
                    ],
                    optional_points: vec!["Lossy removal often affects higher frequencies.".into()],
                    contradictions: vec![
                        "Smaller always means worse quality, including lossless compression.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "formats-allowed-lossy".into(),
                topic_id: "formats".into(),
                prompt: "Which lossy audio formats are allowed for uploads to Orpheus?".into(),
                reference_answer: "MP3, AAC, AC3 and DTS are allowed.".into(),
                source_url: FORMATS.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "MP3 is allowed.".into(),
                        "AAC is allowed.".into(),
                        "AC3 is allowed.".into(),
                        "DTS is allowed.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec!["WMA is an allowed upload format.".into()],
                },
            },
            QuestionDto {
                question_id: "formats-minimum-cbr".into(),
                topic_id: "formats".into(),
                prompt: "What is the minimum CBR MP3 bitrate allowed for uploads to Orpheus?".into(),
                reference_answer: "192 kbps.".into(),
                source_url: FORMATS.into(),
                material: vec![],
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["192 kbps".into()],
                    aliases: vec![
                        "192".into(),
                        "192kbps".into(),
                        "192 kbit/s".into(),
                        "192 kilobits per second".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "mp3-v0".into(),
                topic_id: "mp3".into(),
                prompt: "Which LAME VBR preset has the highest quality?".into(),
                reference_answer: "V0 is highest; V9 is lowest.".into(),
                source_url: MP3.into(),
                material: vec![],
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["V0".into()],
                    aliases: vec!["-V0".into(), "V 0".into()],
                },
            },
            QuestionDto {
                question_id: "mp3-lame-name".into(),
                topic_id: "mp3".into(),
                prompt: "What does LAME stand for?".into(),
                reference_answer: "LAME Ain't An MP3 Encoder.".into(),
                source_url: MP3.into(),
                material: vec![],
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["LAME Ain't An MP3 Encoder".into()],
                    aliases: vec!["LAME Ain’t An MP3 Encoder".into()],
                },
            },
            QuestionDto {
                question_id: "mp3-presets".into(),
                topic_id: "mp3".into(),
                prompt: "What are V0 and V2's target bitrates?".into(),
                reference_answer: "V0 targets about 245 kbps; V2 targets about 190 kbps. Actual averages may differ.".into(),
                source_url: MP3.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "V0 targets about 245 kbps.".into(),
                        "V2 targets about 190 kbps.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "Every V0 encode has exactly 245 kbps average bitrate.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "mp3-v2-allowed".into(),
                topic_id: "mp3".into(),
                prompt: "When is V2 allowed for music torrents on Orpheus?".into(),
                reference_answer: "V2 is allowed only when no better bitrate exists.".into(),
                source_url: MP3.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec!["V2 is allowed only if no better bitrate exists.".into()],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "mp3-abr".into(),
                topic_id: "mp3".into(),
                prompt: "How does ABR differ from CBR and VBR, and why can it be a poor compromise between them?".into(),
                reference_answer: "ABR varies bitrate while targeting a chosen average, so file size is predictable. CBR keeps one bitrate; VBR targets quality. The guide says ABR's compromise performs neither approach well.".into(),
                source_url: MP3.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "ABR varies bitrate while targeting a preset average.".into(),
                        "CBR holds bitrate constant.".into(),
                        "VBR targets quality.".into(),
                        "ABR's compromise performs neither approach well.".into(),
                    ],
                    optional_points: vec![
                        "ABR's target average makes file size predictable.".into(),
                    ],
                    contradictions: vec![
                        "ABR and VBR both guarantee the selected average bitrate.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "transcodes-chain".into(),
                topic_id: "transcodes".into(),
                prompt: "Compare WAV → FLAC → MP3 with WAV → MP3 → AAC. Which is good, and why?".into(),
                reference_answer: "The first chain is good because lossless conversions precede a single final lossy conversion. The second is bad because it re-encodes an already lossy source.".into(),
                source_url: TRANSCODES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "WAV to FLAC to MP3 is good; lossy conversion occurs only at the last step.".into(),
                        "WAV to MP3 to AAC is bad because AAC is made from a lossy source.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "Changing the lossy output format makes another lossy conversion acceptable.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "torrenting-ratio".into(),
                topic_id: "torrenting".into(),
                prompt: "You upload 30 GB and download 15 GB. What is your ratio?".into(),
                reference_answer: "2.0, because uploaded data is divided by downloaded data.".into(),
                source_url: TORRENTING.into(),
                material: vec![],
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["2".into()],
                    aliases: vec!["2.0".into(), "2:1".into()],
                },
            },
            QuestionDto {
                question_id: "torrenting-ratio-balance".into(),
                topic_id: "torrenting".into(),
                prompt: "Why is 1.0 a healthy upload-to-download ratio, and what concerns can ratios far below or above 1.0 raise?".into(),
                reference_answer: "At 1.0, uploaded and downloaded amounts are equal. Far below 1.0 means taking more than you contribute; far above can mean hoarding upload instead of downloading and supporting other peers.".into(),
                source_url: TORRENTING.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "1.0 means equal uploaded and downloaded data.".into(),
                        "Far below means contributing less than you take.".into(),
                        "Far above can mean hoarding upload rather than downloading.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec!["A high ratio automatically breaks a golden rule.".into()],
                },
            },
            QuestionDto {
                question_id: "torrenting-peers".into(),
                topic_id: "torrenting".into(),
                prompt: "What do a torrent file and a client do, and how do peers, leeching and seeding relate?".into(),
                reference_answer: "The torrent file describes the wanted data; a client connects peers sharing it. Leeching downloads pieces and seeding offers them. A peer can upload completed pieces while still downloading.".into(),
                source_url: TORRENTING.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "A torrent file describes the data to be shared.".into(),
                        "A client connects peers sharing the data.".into(),
                        "Leeching downloads data.".into(),
                        "Seeding offers data to peers.".into(),
                    ],
                    optional_points: vec![
                        "A peer can upload completed pieces while downloading other pieces.".into(),
                    ],
                    contradictions: vec![
                        "Seeding cannot begin until every piece is downloaded.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "torrenting-credit".into(),
                topic_id: "torrenting".into(),
                prompt: "Does seeding automatically earn upload credit? Explain why.".into(),
                reference_answer: "No. Upload credit requires actual data transfer to another peer; time spent available to seed alone earns no upload bytes.".into(),
                source_url: TORRENTING.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Upload credit requires actual data transfer to another peer.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec!["Time spent seeding alone earns upload bytes.".into()],
                },
            },
            QuestionDto {
                question_id: "torrenting-seeding-ratio".into(),
                topic_id: "torrenting".into(),
                prompt: "On Orpheus, what required-ratio benefit applies when you seed every downloaded torrent at least 72 hours per week?".into(),
                reference_answer: "The required ratio is zero until total downloads exceed 20 GB.".into(),
                source_url: TORRENTING.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "The required ratio is zero until total downloads exceed 20 GB.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "torrenting-seeding-duration".into(),
                topic_id: "torrenting".into(),
                prompt: "How long should you keep seeding after reaching 72 hours per week?".into(),
                reference_answer: "No. Keep seeding as long as possible; 72 hours is not an instruction to stop.".into(),
                source_url: TORRENTING.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Continue seeding as long as possible; 72 hours is not a stopping point.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "torrenting-watch".into(),
                topic_id: "torrenting".into(),
                prompt: "What happens below required ratio, and how can you recover downloading privileges? Include the ratio-watch download limit.".into(),
                reference_answer: "Ratio watch lasts two weeks. Failure to recover disables downloading; downloading over 10 GB during watch disables it immediately. Continue seeding, fill requests or upload allowed new torrents to improve ratio.".into(),
                source_url: TORRENTING.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Ratio watch gives two weeks to recover before downloads are disabled.".into(),
                        "Over 10 GB downloaded on watch disables downloads immediately.".into(),
                        "Recover by seeding, filling requests or uploading rule-compliant new torrents.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "Duplicates or bad transcodes are acceptable to repair ratio.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "torrenting-leech-types".into(),
                topic_id: "torrenting".into(),
                prompt: "How do freeleech and neutral leech affect recorded upload and download?".into(),
                reference_answer: "Freeleech counts upload but not download. Neutral leech counts neither.".into(),
                source_url: TORRENTING.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Freeleech excludes download from recorded statistics.".into(),
                        "Freeleech counts upload in recorded statistics.".into(),
                        "Neutral leech excludes upload from recorded statistics.".into(),
                        "Neutral leech excludes download from recorded statistics.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec!["Neutral leech counts upload toward ratio.".into()],
                },
            },
            QuestionDto {
                question_id: "torrenting-connectability".into(),
                topic_id: "torrenting".into(),
                prompt: "Why can port forwarding help a BitTorrent client behind NAT?".into(),
                reference_answer: "NAT can block incoming connections. Forward the client's port to the computer running it so outside peers can connect, increasing opportunities to upload.".into(),
                source_url: TORRENTING.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Port forwarding routes incoming peer connections through NAT to the computer running the client, increasing opportunities to upload.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "Port forwarding guarantees upload credit without data transfer.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "torrenting-private".into(),
                topic_id: "torrenting".into(),
                prompt: "How should DHT and the private-torrent flag be configured for Orpheus, and why?".into(),
                reference_answer: "Disable DHT and mark created torrents private. DHT discovers peers without the private tracker; these settings help keep nonmembers out of the peer list.".into(),
                source_url: TORRENTING.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Disable DHT.".into(),
                        "Mark uploaded torrents private.".into(),
                        "These settings exclude discovery of nonmember peers outside the tracker.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec!["Enable DHT to share Orpheus torrents publicly.".into()],
                },
            },
            QuestionDto {
                question_id: "torrenting-partial".into(),
                topic_id: "torrenting".into(),
                prompt: "Why can a large torrent show many leechers without being a good way to gain ratio?".into(),
                reference_answer: "Some peers are partial seeders. They have only part of the torrent and do not intend to download the rest, so they look like leechers without providing the expected demand.".into(),
                source_url: TORRENTING.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Partial seeders appear as leechers because they hold only part of the torrent, but they do not intend to download the rest.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "Every listed leecher intends to download the whole torrent.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "torrenting-whitelist".into(),
                topic_id: "torrenting".into(),
                prompt: "What does a whitelisted client version written as 4.6.x mean?".into(),
                reference_answer: "The x accepts any number in that position, such as 4.6.0 or 4.6.2.".into(),
                source_url: TORRENTING.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "x is a wildcard for any number in that position.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "Approval of one version means all future versions are approved.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "torrenting-client-approval".into(),
                topic_id: "torrenting".into(),
                prompt: "Why should you check the client whitelist before choosing a BitTorrent client version for Orpheus?".into(),
                reference_answer: "Only approved client versions can connect to Orpheus peers.".into(),
                source_url: TORRENTING.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Only approved client versions can connect to Orpheus peers.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "cd-ripping-program".into(),
                topic_id: "cd".into(),
                prompt: "Which CD-ripping program is recommended for quality rips and upload logs on Orpheus when using macOS?".into(),
                reference_answer: "XLD. EAC is recommended for Windows or Linux with Wine.".into(),
                source_url: CD.into(),
                material: vec![],
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["XLD".into()],
                    aliases: vec!["X Lossless Decoder".into()],
                },
            },
            QuestionDto {
                question_id: "cd-burn-rip".into(),
                topic_id: "cd".into(),
                prompt: "What is the difference between CD ripping and CD burning?".into(),
                reference_answer: "Ripping extracts music from a CD; burning makes a CD from music files.".into(),
                source_url: CD.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Ripping extracts music from a CD.".into(),
                        "Burning writes music files to a CD.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "Burning a lossy file restores the original recording.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "cd-ripping-eac".into(),
                topic_id: "cd".into(),
                prompt: "Which CD-ripping program is recommended for quality rips and upload logs on Orpheus when using Windows or Linux with Wine?".into(),
                reference_answer: "Exact Audio Copy, abbreviated EAC.".into(),
                source_url: CD.into(),
                material: vec![],
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["EAC".into()],
                    aliases: vec!["Exact Audio Copy".into(), "Exact Audio Copy (EAC)".into()],
                },
            },
            QuestionDto {
                question_id: "cd-log-integrity".into(),
                topic_id: "cd".into(),
                prompt: "May you edit a ripping log for any reason, including to remove an error?".into(),
                reference_answer: "No. Editing a ripping log for any reason is prohibited.".into(),
                source_url: CD.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec!["Editing a log for any reason is prohibited.".into()],
                    optional_points: vec![],
                    contradictions: vec!["Logs may be cleaned up before upload.".into()],
                },
            },
            QuestionDto {
                question_id: "cd-log-role".into(),
                topic_id: "cd".into(),
                prompt: "What does a ripping log record?".into(),
                reference_answer: "It records the ripping process and any errors that occurred.".into(),
                source_url: CD.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "A log records the ripping process and any errors.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "cd-cue-role".into(),
                topic_id: "cd".into(),
                prompt: "What does a cue sheet provide when burning a copy of a CD?".into(),
                reference_answer: "It is a text catalog or table of contents describing the CD layout, allowing a CD identical to the original to be burned.".into(),
                source_url: CD.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "A cue supplies the CD layout information needed to recreate the original CD.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec!["A cue contains the audio samples themselves.".into()],
                },
            },
            QuestionDto {
                question_id: "dupes-trumping".into(),
                topic_id: "dupes".into(),
                prompt: "What is replacing a torrent that fails the rules with one that follows them called?".into(),
                reference_answer: "Trumping.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["Trumping".into()],
                    aliases: vec!["A trump".into(), "Trump".into()],
                },
            },
            QuestionDto {
                question_id: "dupes-editions".into(),
                topic_id: "dupes".into(),
                prompt: "Do a different catalog number and packaging alone make a distinct edition? What evidence matters?".into(),
                reference_answer: "No. Different catalog numbers or packaging alone do not establish a distinct edition. The audio content must differ significantly and meaningfully, for example through bonus tracks. Compare tracklists and running order, along with ripping-log details such as the table of contents (TOC), peak levels and pre-gaps, for evidence of those differences.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Catalog or packaging differences alone do not establish a distinct edition.".into(),
                        "A distinct edition requires significantly and meaningfully different audio content.".into(),
                        "Identify tracklist or running order as concrete evidence to compare. Stating only that the audio must differ does not cover this evidence requirement.".into(),
                        "Identify at least one ripping-log detail such as TOC, peak levels or pre-gaps as concrete evidence to compare. Stating only that the audio must differ does not cover this evidence requirement.".into(),
                    ],
                    optional_points: vec![
                        "Bonus tracks may establish different audio content.".into(),
                    ],
                    contradictions: vec!["Every different pressing is allowed to coexist.".into()],
                },
            },
            QuestionDto {
                question_id: "dupes-same-quality".into(),
                topic_id: "dupes".into(),
                prompt: "When is a torrent considered a duplicate of an existing torrent?".into(),
                reference_answer: "A torrent is a duplicate when it matches an existing torrent's edition, format and bitrate, with a comparable or identical sampling rate.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Same edition and same quality/format/bitrate with comparable sampling rate is a dupe.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "dupes-scene-status".into(),
                topic_id: "dupes".into(),
                prompt: "Does a scene label affect whether a torrent is considered a duplicate?".into(),
                reference_answer: "No. Scene and non-scene torrents are duplicates if they are otherwise identical.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Scene status alone does not avoid duplication.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec!["Scene and non-scene copies always coexist.".into()],
                },
            },
            QuestionDto {
                question_id: "dupes-inactive".into(),
                topic_id: "dupes".into(),
                prompt: "What can happen to a torrent unseeded for two weeks, and what should you do if you have its original files?".into(),
                reference_answer: "It can be trumped by an identical reseed or new rip/encode. If you have the original files, reseed the original torrent instead of uploading a new one.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "A torrent unseeded for two weeks can be trumped.".into(),
                        "Reseed its original torrent if you have the files.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "A briefly unseeded torrent automatically permits a duplicate upload.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "dupes-lossy-hierarchy".into(),
                topic_id: "dupes".into(),
                prompt: "How do higher CBR/ABR bitrates, MP3 320 CBR and V0 affect lossy trumps?".into(),
                reference_answer: "Higher CBR replaces lower CBR; higher ABR replaces lower ABR. MP3 320 CBR or V0 trumps CBR below 320.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Higher CBR trumps lower CBR.".into(),
                        "Higher ABR trumps lower ABR.".into(),
                        "MP3 320 CBR and V0 trump CBR below 320.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec!["V0 always trumps 320 CBR.".into()],
                },
            },
            QuestionDto {
                question_id: "dupes-320-v0-coexist".into(),
                topic_id: "dupes".into(),
                prompt: "Can MP3 320 CBR and V0 coexist for the same edition on Orpheus?".into(),
                reference_answer: "Yes. Both may coexist, provided no duplicate rip with the same bitrate and format already exists.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "MP3 320 CBR and V0 may coexist, provided neither duplicates an existing rip in its bitrate and format.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec!["V0 always trumps 320 CBR.".into()],
                },
            },
            QuestionDto {
                question_id: "dupes-vbr-hierarchy".into(),
                topic_id: "dupes".into(),
                prompt: "Under Orpheus's trump rules, which current VBR presets can replace MP3 APS, APX and V1?".into(),
                reference_answer: "V2 replaces APS; V0 replaces APX and V1. V2 and V0 are separate protected endpoints in the chart, subject to the guide's upload restrictions.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "V2 replaces APS.".into(),
                        "V0 replaces APX.".into(),
                        "V0 replaces V1.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec!["APS and APX can never be trumped.".into()],
                },
            },
            QuestionDto {
                question_id: "dupes-aac-extras".into(),
                topic_id: "dupes".into(),
                prompt: "When can MP3 trump AAC on Orpheus?".into(),
                reference_answer: "Allowed MP3 can trump AAC of the same edition and media, except iTunes purchases with iTunes Exclusive tracks.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Allowed MP3 trumps equivalent AAC except the iTunes Exclusive tracks exception.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "AAC with iTunes Exclusive tracks can always be trumped by MP3.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "dupes-lossy-extras".into(),
                topic_id: "dupes".into(),
                prompt: "Do log files, cue files, playlists or artwork let a lossy torrent trump an equivalent one?".into(),
                reference_answer: "No. These extra files do not permit an equivalent lossy torrent to trump an existing one.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Extra files do not trump equivalent lossy audio.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec!["Adding a cue always permits a lossy trump.".into()],
                },
            },
            QuestionDto {
                question_id: "dupes-cd-source".into(),
                topic_id: "dupes".into(),
                prompt: "Can a rip from a CD-R copy of a commercial pressed CD replace the original CD rip? State the CD-R exception.".into(),
                reference_answer: "No. Rips must come from commercially pressed or official artist/label-approved CDs. The exception is a release distributed only on CD-R by the artist or label.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "A CD-R copy of a pressed CD is not an allowed source.".into(),
                        "Artist/label releases distributed only on CD-R are an exception.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "Burning and re-ripping a pressed CD is an equivalent approved source.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "dupes-log-scores".into(),
                topic_id: "dupes".into(),
                prompt: "Compare an approved-tool log versus no log, a 100% log versus 80%, and a 99% log versus 80%. Which can trump?".into(),
                reference_answer: "An EAC/XLD log of any score can trump no log or a nonapproved-tool log. A 100% log can trump 80%; 99% cannot trump an existing 80% approved-tool log.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "An approved-tool log can trump no log.".into(),
                        "A 100% log can trump an 80% log.".into(),
                        "A 99% log cannot trump an 80% approved-tool log.".into(),
                    ],
                    optional_points: vec![
                        "An approved-tool log can also trump a nonapproved-tool log.".into(),
                    ],
                    contradictions: vec!["Any higher log score can trump a lower one.".into()],
                },
            },
            QuestionDto {
                question_id: "dupes-cue-trump".into(),
                topic_id: "dupes".into(),
                prompt: "When can a 100% log rip with a cue trump a 100% log rip without one?".into(),
                reference_answer: "The noncompliant cue must materially differ from a cue generated from the log, such as containing correct extra indices, detected pre-gap lengths or pre-emphasis flags.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "The cue must materially differ from one generated from the log.".into(),
                    ],
                    optional_points: vec![
                        "Material differences can include extra indices, detected pre-gaps or pre-emphasis flags.".into(),
                    ],
                    contradictions: vec!["Adding any generated cue is enough to trump.".into()],
                },
            },
            QuestionDto {
                question_id: "dupes-tags".into(),
                topic_id: "dupes".into(),
                prompt: "What distinguishes a tag trump from a minor spelling fix?".into(),
                reference_answer: "Missing or completely wrong required fields can be trumped with substantial corrections; tiny spelling or diacritic fixes are insufficient.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "A tag trump requires substantial correction, not tiny spelling or diacritic changes.".into(),
                    ],
                    optional_points: vec!["Album titles must use proper capitalization.".into()],
                    contradictions: vec!["Any typo permits a tag trump.".into()],
                },
            },
            QuestionDto {
                question_id: "dupes-required-tags".into(),
                topic_id: "dupes".into(),
                prompt: "Which audio tag fields are required for uploads to Orpheus?".into(),
                reference_answer: "Title, album, artist and track number.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "The title tag is required.".into(),
                        "The album tag is required.".into(),
                        "The artist tag is required.".into(),
                        "The track number tag is required.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "dupes-folders".into(),
                topic_id: "dupes".into(),
                prompt: "What folder and filename rules should a music upload follow, including a single track and a multi-disc release?".into(),
                reference_answer: "Put music in a meaningfully named folder containing at least the album name, even for one track. Avoid archives and needless nesting; filenames must reflect titles. Separate discs into folders if track numbers repeat.".into(),
                source_url: DUPES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Use an album-named folder even for a single track.".into(),
                        "Do not put music files in archives.".into(),
                        "Avoid unnecessary folder nesting.".into(),
                        "Filenames must reflect track titles.".into(),
                        "Do not put repeated track numbers from multiple discs in one directory.".into(),
                    ],
                    optional_points: vec![
                        "Year and format in the folder name are encouraged.".into(),
                    ],
                    contradictions: vec!["A ZIP archive is an acceptable music folder.".into()],
                },
            },
            QuestionDto {
                question_id: "spectral-axes".into(),
                topic_id: "spectral".into(),
                prompt: "What do this spectral diagram's horizontal and vertical axes represent?".into(),
                reference_answer: "The horizontal axis shows time; the vertical axis shows frequency in Hz/kHz.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-a.png".into(),
                    description: "Spectral diagram A with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "The horizontal axis shows time.".into(),
                        "The vertical axis shows frequency in Hz/kHz.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-lossless".into(),
                topic_id: "spectral".into(),
                prompt: "How high does the visible content extend, in kHz?".into(),
                reference_answer: "The visible content extends to about 22 kHz.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-a.png".into(),
                    description: "Spectral diagram A with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "The visible content extends to about 22 kHz, including equivalent values in Hz.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-hearing-range".into(),
                topic_id: "spectral".into(),
                prompt: "Does this recording contain frequencies above the typical human hearing range? Explain using the diagram.".into(),
                reference_answer: "Yes. The content reaches about 22 kHz, above the typical upper hearing limit of about 20 kHz.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-a.png".into(),
                    description: "Spectral diagram A with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "The content reaches about 22 kHz, above the typical upper hearing limit of about 20 kHz.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-lossless-conversion".into(),
                topic_id: "spectral".into(),
                prompt: "Would converting lossless audio between FLAC, WAV and ALAC change its spectrum? Explain why.".into(),
                reference_answer: "No. Lossless-to-lossless conversion preserves the audio data, so its spectrum stays the same.".into(),
                source_url: SPECTRALS.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Lossless-to-lossless conversion preserves the audio data and spectrum.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec!["Lossless conversion removes high-frequency data.".into()],
                },
            },
            QuestionDto {
                question_id: "spectral-classical".into(),
                topic_id: "spectral".into(),
                prompt: "Does this piano recording's musical spectrum establish a lossy transcode? Explain.".into(),
                reference_answer: "No. Piano music can naturally contain little high-frequency musical energy, so a sparse musical spectrum alone does not establish a lossy transcode.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-b.jpg".into(),
                    description: "Classical piano spectral diagram from the Orpheus interview guide, with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "No. Piano music can naturally contain little high-frequency musical energy, so musical sparsity alone does not establish a lossy transcode.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "All lossless music must have strong energy across every frequency.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "spectral-piano-noise".into(),
                topic_id: "spectral".into(),
                prompt: "Which feature of this piano spectrum supports a lossless source?".into(),
                reference_answer: "Faint background noise extends to about 22 kHz, showing that high-frequency content is still present.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-k.png".into(),
                    description: "Spectral diagram K of the supplied piano recording, with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Faint background noise extending to about 22 kHz supports a lossless source.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-piano-transcode".into(),
                topic_id: "spectral".into(),
                prompt: "This piano recording was converted from MP3 to FLAC. Does that recover discarded audio data? Explain.".into(),
                reference_answer: "No. MP3 encoding discarded audio data that conversion to FLAC cannot recover.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-l.png".into(),
                    description: "Spectral diagram L of the supplied piano excerpt after the stated conversion, with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Converting MP3 to FLAC cannot recover audio data discarded by the MP3 encode.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "Converting MP3 to FLAC restores the removed frequencies.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "spectral-320".into(),
                topic_id: "spectral".into(),
                prompt: "How high do the tallest purple spikes reach, in kHz?".into(),
                reference_answer: "The tallest purple spikes reach about 20 kHz. The continuous band ends near 16 kHz.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-c.png".into(),
                    description: "Spectral diagram C with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "The tallest purple spikes reach around 20 kHz. Accept rough estimates within 19.5 to 20.5 kHz, including equivalent values in Hz.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-320-comparison".into(),
                topic_id: "spectral".into(),
                prompt: "Are the purple spike tips below, near, or above the guide's MP3 320 kbps CBR cutoff? State the guide's cutoff in kHz as part of your answer.".into(),
                reference_answer: "The spike tips are near the 20.5 kHz reference, or slightly below it. The spikes reach about 20 kHz; the guide's cutoff is 20.5 kHz.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-c.png".into(),
                    description: "Spectral diagram C with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Explicitly state the guide's MP3 320 kbps CBR cutoff of 20.5 kHz, or its equivalent in Hz. A comparison alone is insufficient.".into(),
                        "The spike tips are near the 20.5 kHz reference, or slightly below it. Accept 'near' or the stated slight difference, including 'below' or 'above' paired with an accurate frequency estimate. A claim that the spikes are far below or far above the reference does not cover this point. A numerical estimate of the spike height is not required.".into(),
                    ],
                    optional_points: vec![
                        "The tallest purple spikes reach around 20 kHz. Accept rough estimates within 19.5 to 20.5 kHz, including equivalent values in Hz.".into(),
                    ],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-256".into(),
                topic_id: "spectral".into(),
                prompt: "How high do the tallest purple spikes reach, in kHz?".into(),
                reference_answer: "The tallest purple spikes reach about 19.5 kHz. The continuous band ends near 16 kHz.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-d.png".into(),
                    description: "Spectral diagram D with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "The tallest purple spikes reach around 19.5 kHz. Accept rough estimates within 19 to 20 kHz, including equivalent values in Hz.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-256-comparison".into(),
                topic_id: "spectral".into(),
                prompt: "Are the purple spike tips below, near, or above the guide's MP3 256 kbps CBR cutoff? State the guide's cutoff in kHz as part of your answer.".into(),
                reference_answer: "The spike tips are near the 20 kHz reference, or slightly below it. The spikes reach about 19.5 kHz; the guide's cutoff is 20 kHz.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-d.png".into(),
                    description: "Spectral diagram D with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Explicitly state the guide's MP3 256 kbps CBR cutoff of 20 kHz, or its equivalent in Hz. A comparison alone is insufficient.".into(),
                        "The spike tips are near the 20 kHz reference, or slightly below it. Accept 'near' or the stated slight difference, including 'below' or 'above' paired with an accurate frequency estimate. A claim that the spikes are far below or far above the reference does not cover this point. A numerical estimate of the spike height is not required.".into(),
                    ],
                    optional_points: vec![
                        "The tallest purple spikes reach around 19.5 kHz. Accept rough estimates within 19 to 20 kHz, including equivalent values in Hz.".into(),
                    ],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-192".into(),
                topic_id: "spectral".into(),
                prompt: "How high do the tallest purple spikes reach, in kHz?".into(),
                reference_answer: "The tallest purple spikes reach about 18.5 to 19 kHz. The continuous band ends near 16 kHz.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-f.png".into(),
                    description: "Spectral diagram F with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "The tallest purple spikes reach around 18.5 to 19 kHz. Accept rough estimates within 18 to 19.5 kHz, including equivalent values in Hz.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-192-comparison".into(),
                topic_id: "spectral".into(),
                prompt: "Are the purple spike tips below, near, or above the guide's MP3 192 kbps CBR cutoff? State the guide's cutoff in kHz as part of your answer.".into(),
                reference_answer: "The spike tips are near the 19 kHz reference, or slightly below it. The spikes reach about 18.5 to 19 kHz; the guide's cutoff is 19 kHz.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-f.png".into(),
                    description: "Spectral diagram F with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Explicitly state the guide's MP3 192 kbps CBR cutoff of 19 kHz, or its equivalent in Hz. A comparison alone is insufficient.".into(),
                        "The spike tips are near the 19 kHz reference, or slightly below it. Accept 'near' or the stated slight difference, including 'below' or 'above' paired with an accurate frequency estimate. A claim that the spikes are far below or far above the reference does not cover this point. A numerical estimate of the spike height is not required.".into(),
                    ],
                    optional_points: vec![
                        "The tallest purple spikes reach around 18.5 to 19 kHz. Accept rough estimates within 18 to 19.5 kHz, including equivalent values in Hz.".into(),
                    ],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-v2".into(),
                topic_id: "spectral".into(),
                prompt: "How high do the tallest purple spikes reach, in kHz?".into(),
                reference_answer: "The tallest purple spikes reach about 18.5 to 19 kHz. The continuous band ends near 16 kHz.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-g.png".into(),
                    description: "Spectral diagram G with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "The tallest purple spikes reach around 18.5 to 19 kHz. Accept rough estimates within 18 to 19.5 kHz, including equivalent values in Hz.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-v2-comparison".into(),
                topic_id: "spectral".into(),
                prompt: "Are the purple spike tips below, near, or above the guide's MP3 V2 cutoff? State the guide's cutoff in kHz as part of your answer.".into(),
                reference_answer: "The spike tips are near the 18.5 kHz reference, or slightly above it. The spikes reach about 18.5 to 19 kHz; the guide's cutoff is 18.5 kHz.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-g.png".into(),
                    description: "Spectral diagram G with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Explicitly state the guide's MP3 V2 cutoff of 18.5 kHz, or its equivalent in Hz. A comparison alone is insufficient.".into(),
                        "The spike tips are near the 18.5 kHz reference, or slightly above it. Accept 'near' or the stated slight difference, including 'below' or 'above' paired with an accurate frequency estimate. A claim that the spikes are far below or far above the reference does not cover this point. A numerical estimate of the spike height is not required.".into(),
                    ],
                    optional_points: vec![
                        "The tallest purple spikes reach around 18.5 to 19 kHz. Accept rough estimates within 18 to 19.5 kHz, including equivalent values in Hz.".into(),
                    ],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-128".into(),
                topic_id: "spectral".into(),
                prompt: "How high do the tallest purple spikes reach, in kHz?".into(),
                reference_answer: "The tallest purple spikes reach about 16 to 17 kHz. The continuous band ends near 16 kHz.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-h.png".into(),
                    description: "Spectral diagram H with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "The tallest purple spikes reach around 16 to 17 kHz. Accept rough estimates within 16 to 17 kHz, including equivalent values in Hz.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-128-comparison".into(),
                topic_id: "spectral".into(),
                prompt: "Are the purple spike tips below, near, or above the guide's MP3 128 kbps CBR cutoff? State the guide's cutoff in kHz as part of your answer.".into(),
                reference_answer: "The spike tips are near the 16 kHz reference, or slightly above it. The spikes reach about 16 to 17 kHz; the guide's cutoff is 16 kHz.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-h.png".into(),
                    description: "Spectral diagram H with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Explicitly state the guide's MP3 128 kbps CBR cutoff of 16 kHz, or its equivalent in Hz. A comparison alone is insufficient.".into(),
                        "The spike tips are near the 16 kHz reference, or slightly above it. Accept 'near' or the stated slight difference, including 'below' or 'above' paired with an accurate frequency estimate. A claim that the spikes are far below or far above the reference does not cover this point. A numerical estimate of the spike height is not required.".into(),
                    ],
                    optional_points: vec![
                        "The tallest purple spikes reach around 16 to 17 kHz. Accept rough estimates within 16 to 17 kHz, including equivalent values in Hz.".into(),
                    ],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-v0".into(),
                topic_id: "spectral".into(),
                prompt: "How high do the purple streaks extend, in kHz?".into(),
                reference_answer: "The purple streaks reach about 21 to 22 kHz.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-e.png".into(),
                    description: "Spectral diagram E with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "The purple streaks reach approximately 21 to 22 kHz, including equivalent values in Hz.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-v0-comparison".into(),
                topic_id: "spectral".into(),
                prompt: "Are the purple streak tips below, near, or above the guide's MP3 V0 cutoff? State the guide's cutoff in kHz as part of your answer.".into(),
                reference_answer: "Above. The purple streaks reach about 21 to 22 kHz, above the guide's 19.5 kHz cutoff.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-e.png".into(),
                    description: "Spectral diagram E with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Explicitly state the guide's MP3 V0 cutoff of 19.5 kHz, or its equivalent in Hz. A comparison alone is insufficient.".into(),
                        "The purple streak tips extend above the guide's MP3 V0 cutoff. A numerical estimate of the streak height is not required.".into(),
                    ],
                    optional_points: vec!["The purple streaks reach about 21 to 22 kHz.".into()],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-mp3-shelf".into(),
                topic_id: "spectral".into(),
                prompt: "At approximately what frequency do MP3 spectrums commonly show a shelf?".into(),
                reference_answer: "About 16 kHz.".into(),
                source_url: SPECTRALS.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec!["MP3s commonly show a shelf near 16 kHz.".into()],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "spectral-disguised-flac".into(),
                topic_id: "spectral".into(),
                prompt: "This file is encoded as FLAC. What can you infer about its source from the spectrum? Explain your reasoning.".into(),
                reference_answer: "The cutoff near 18.5 to 19 kHz suggests a lossy source despite the FLAC format. It is close to the guide's 19 kHz MP3 192 kbps CBR reference. This example was made by converting MP3 192 kbps CBR to FLAC.".into(),
                source_url: SPECTRALS.into(),
                material: vec![ExerciseMaterialDto::Image {
                    src: "/exercises/spectrum-i.png".into(),
                    description: "Spectral diagram I for the supplied file, with time and frequency axes.".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "The cutoff near 18.5 to 19 kHz suggests a lossy source despite the FLAC format.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "A FLAC extension or large file size proves a lossless source.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "cd-cue-reading".into(),
                topic_id: "cd".into(),
                prompt: "Read the track layout in this cue excerpt: identify the audio track number, pre-gap and index position.".into(),
                reference_answer: "It describes audio track 01, with PREGAP 00:00:32 and INDEX 01 at 00:00:00.".into(),
                source_url: "https://interview.orpheus.network/cue-file.php".into(),
                material: vec![ExerciseMaterialDto::Excerpt {
                    description: "Selected fields from the official sample cue; omitted lines are not shown.".into(),
                    text: "  TRACK 01 AUDIO\n    PREGAP 00:00:32\n    INDEX 01 00:00:00".into(),
                }],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "The excerpt describes AUDIO track 01.".into(),
                        "PREGAP is 00:00:32.".into(),
                        "INDEX 01 is 00:00:00.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "rules-disabled".into(),
                topic_id: "rules".into(),
                prompt: "Which IRC channel should a person with a disabled Orpheus account use?".into(),
                reference_answer: "#disabled. Do not create another account or interview again.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::ShortAnswer {
                    accepted_answers: vec!["#disabled".into()],
                    aliases: vec!["disabled".into()],
                },
            },
            QuestionDto {
                question_id: "rules-account".into(),
                topic_id: "rules".into(),
                prompt: "How many Orpheus accounts may one person have over their lifetime?".into(),
                reference_answer: "One account per person per lifetime, even if the account is disabled.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec!["One account per person per lifetime.".into()],
                    optional_points: vec![],
                    contradictions: vec!["A disabled account permits a new account.".into()],
                },
            },
            QuestionDto {
                question_id: "rules-account-sharing".into(),
                topic_id: "rules".into(),
                prompt: "May you transfer or share your Orpheus account?".into(),
                reference_answer: "No. Accounts are for personal use only and may not be traded, sold, given away or shared.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec!["No transfer or sharing of accounts.".into()],
                    optional_points: vec!["Ask staff to disable an unwanted account.".into()],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "rules-account-activity".into(),
                topic_id: "rules".into(),
                prompt: "What must you do to keep your Orpheus account active?".into(),
                reference_answer: "Log into the site regularly to keep the account in good standing.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec!["Log into the site regularly.".into()],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "rules-invitees".into(),
                topic_id: "rules".into(),
                prompt: "Under Orpheus's invite rules, which invitee behavior can put your account at risk, and what invitee failure is exempt?".into(),
                reference_answer: "Invite only people you know and trust. Invitee golden-rule violations threaten your invite privileges and account; failing required ratio alone is exempt.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Invitees' golden-rule breaches can endanger the inviter's privileges or account.".into(),
                        "Invitees' ratio failure alone is not punished against the inviter.".into(),
                    ],
                    optional_points: vec!["Invite only people you know and trust.".into()],
                    contradictions: vec!["Inviters are never responsible for invitees.".into()],
                },
            },
            QuestionDto {
                question_id: "rules-invites".into(),
                topic_id: "rules".into(),
                prompt: "What restrictions apply to offering and requesting invites, including the forum and recruiter exceptions?".into(),
                reference_answer: "Do not sell, trade or publicly offer invites, except staff-designated recruiters in approved places. Do not request invites/accounts or send unsolicited requests. Eligible users may respond privately to an offer in the Invites Forum.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Do not trade invites.".into(),
                        "Do not sell invites.".into(),
                        "Do not publicly offer invites, except as a designated recruiter in approved locations.".into(),
                        "Do not request invites or accounts.".into(),
                        "Do not send unsolicited invite requests.".into(),
                        "Messaging in response to an Invites Forum offer is permitted.".into(),
                    ],
                    optional_points: vec![
                        "The Invites Forum is restricted to Elite and above.".into(),
                    ],
                    contradictions: vec!["Private unsolicited requests are allowed.".into()],
                },
            },
            QuestionDto {
                question_id: "rules-ratio".into(),
                topic_id: "rules".into(),
                prompt: "Why are buffer transfers through unintended site features and false tracker reports prohibited?".into(),
                reference_answer: "Unintended buffer transfer or inflation is ratio manipulation. False tracker reports are cheating, even with an approved client. Ask staff if unsure.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Unintended buffer transfer/increase is ratio manipulation.".into(),
                        "Reporting false data is cheating even with an approved client.".into(),
                    ],
                    optional_points: vec!["Ask staff about uncertainty.".into()],
                    contradictions: vec![
                        "An approved client makes manipulated reports acceptable.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "rules-client".into(),
                topic_id: "rules".into(),
                prompt: "What requirements must your BitTorrent client meet for normal use on Orpheus?".into(),
                reference_answer: "Use an unmodified whitelisted client with direct, exclusive access.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "The client must be whitelisted.".into(),
                        "The client must be unmodified.".into(),
                        "You must have direct, exclusive access to the client.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "A modified approved client is automatically approved.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "rules-client-testing".into(),
                topic_id: "rules".into(),
                prompt: "What must a developer do before testing an unstable client on Orpheus?".into(),
                reference_answer: "Obtain staff approval before testing.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Unstable-client testing requires prior staff approval.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "rules-torrent-passkey".into(),
                topic_id: "rules".into(),
                prompt: "Why must you not share Orpheus torrent files?".into(),
                reference_answer: "Each torrent file contains an announce URL with your personal passkey. Sharing the file exposes that passkey.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Sharing torrent files exposes your personal passkey.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "The torrent file is safe to share if your password is withheld.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "rules-announce-urls".into(),
                topic_id: "rules".into(),
                prompt: "Why must you not add other announce URLs to Orpheus torrent files?".into(),
                reference_answer: "Adding outside announce URLs is prohibited because it can cause false tracker reporting interpreted as cheating.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Adding outside announce URLs can cause false tracker reporting interpreted as cheating.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "rules-privacy".into(),
                topic_id: "rules".into(),
                prompt: "What restrictions protect other users' private information, including investigations and automated collection?".into(),
                reference_answer: "Do not expose, threaten or blackmail users with private information. Do not share unvolunteered information without permission or harvest identifying data with scripts/exploits, including information found by investigating public details.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Do not disclose unvolunteered private information without permission.".into(),
                        "Do not use private information for threats or blackmail.".into(),
                        "Do not harvest identifying data with scripts or exploits.".into(),
                        "Privacy protection also covers information found by investigating public details.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "Information found through a search may always be published.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "rules-conduct".into(),
                topic_id: "rules".into(),
                prompt: "How should you respond to a suspected rule breach on Orpheus?".into(),
                reference_answer: "Report suspected breaches rather than confronting or policing users.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Report suspected breaches rather than backseat moderating.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "Confronting a suspected rule breaker is the preferred response.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "rules-scams".into(),
                topic_id: "rules".into(),
                prompt: "What is Orpheus's rule on scams and phishing?".into(),
                reference_answer: "Scams and phishing of any kind are prohibited.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec!["Scams and phishing are prohibited.".into()],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "rules-impersonation".into(),
                topic_id: "rules".into(),
                prompt: "Where does Orpheus's ban on impersonating staff or official service accounts apply?".into(),
                reference_answer: "Staff and official service accounts may not be impersonated on-site, off-site or on IRC.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "No staff/service impersonation on-site, off-site or IRC.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "rules-staff".into(),
                topic_id: "rules".into(),
                prompt: "How should you challenge a staff decision or seek clarification of a rule?".into(),
                reference_answer: "Discuss privately with the deciding moderator, or Staff PM if unavailable. Do not shop around moderators; an administrator can give a second opinion. Respect volunteers and accept staff's final rule interpretation; ask via Staff PM for clarification.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Challenge a decision privately with the deciding moderator, or Staff PM if unavailable.".into(),
                        "Seek rule clarification via Staff PM.".into(),
                        "Do not shop around moderators for a favorable answer.".into(),
                        "An administrator may give a second opinion.".into(),
                    ],
                    optional_points: vec![
                        "Respect staff and their final rule interpretation.".into(),
                    ],
                    contradictions: vec!["User consensus overrides staff's interpretation.".into()],
                },
            },
            QuestionDto {
                question_id: "rules-events-commercial".into(),
                topic_id: "rules".into(),
                prompt: "May users request special events on Orpheus?".into(),
                reference_answer: "No. Special events are staff's decision, follow no fixed schedule and may not be requested by users.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec!["Users cannot request special events.".into()],
                    optional_points: vec![
                        "Special events are staff's decision and follow no fixed schedule.".into(),
                    ],
                    contradictions: vec!["Freeleech must happen on a fixed schedule.".into()],
                },
            },
            QuestionDto {
                question_id: "rules-commercial".into(),
                topic_id: "rules".into(),
                prompt: "What commercial uses of Orpheus services or user data are prohibited?".into(),
                reference_answer: "No. Commercializing services or user data, referral schemes, solicitations and money offers are prohibited.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Commercial gain is prohibited.".into(),
                        "Referral schemes are prohibited.".into(),
                        "Solicitations are prohibited.".into(),
                        "Money offers are prohibited.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![],
                },
            },
            QuestionDto {
                question_id: "rules-vpn".into(),
                topic_id: "rules".into(),
                prompt: "Which VPN/proxy services may be used to browse Orpheus, and is Tor permitted?".into(),
                reference_answer: "Use paid services, including self-hosted services or a seedbox's VPN/proxy. Free services and Tor browsing are prohibited. Ask staff when uncertain.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Use only paid, self-hosted or seedbox VPN/proxy services; free services are prohibited.".into(),
                        "Tor browsing is prohibited.".into(),
                    ],
                    optional_points: vec!["Ask staff when uncertain.".into()],
                    contradictions: vec!["Any free VPN is allowed.".into()],
                },
            },
            QuestionDto {
                question_id: "rules-automation".into(),
                topic_id: "rules".into(),
                prompt: "How must automated access to Orpheus work, and what rate limit applies?".into(),
                reference_answer: "Use the API, at most five requests in any ten-second window. Automated processes must not scrape site HTML.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Automated access uses the API, not HTML scraping.".into(),
                        "At most five requests within any ten-second window.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "Staying under the rate limit permits HTML scraping.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "rules-freeleech-autosnatch".into(),
                topic_id: "rules".into(),
                prompt: "May automated processes snatch freeleech torrents with little or no user input?".into(),
                reference_answer: "No. Freeleech autosnatching with little or no user input is prohibited, including through the API.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "No freeleech autosnatching with little or no user input.".into(),
                    ],
                    optional_points: vec![],
                    contradictions: vec![
                        "Using the API within its rate limit permits freeleech autosnatching.".into(),
                    ],
                },
            },
            QuestionDto {
                question_id: "rules-bugs".into(),
                topic_id: "rules".into(),
                prompt: "What should you do about a live-site bug, and what restrictions apply to exploits?".into(),
                reference_answer: "Do not seek or exploit live bugs. Immediately report critical vulnerabilities under the Responsible Disclosure Policy; report noncritical bugs in the Bugs Forum. Do not publish, share or facilitate exploits; staff decide their classification.".into(),
                source_url: RULES.into(),
                material: vec![],
                grading: AnswerCriteria::Explanation {
                    required_points: vec![
                        "Do not seek or exploit live bugs.".into(),
                        "Report critical bugs through responsible disclosure.".into(),
                        "Report noncritical bugs in the Bugs Forum.".into(),
                        "Do not publish, share or facilitate exploits.".into(),
                    ],
                    optional_points: vec!["Staff decide exploit classifications.".into()],
                    contradictions: vec![
                        "Testing a live vulnerability is allowed before reporting it.".into(),
                    ],
                },
            },
        ],
    }
}

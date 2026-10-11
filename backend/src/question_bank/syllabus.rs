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

pub(super) fn extend(bank: &mut QuestionBank) {
    bank.topics.extend([
        topic("sources", "Analog and digital music sources"),
        topic("torrenting", "Torrenting"),
        topic("spectral", "Spectral analysis"),
        topic("cd", "CD burning and ripping"),
        topic("dupes", "Dupes and trumps"),
        topic("rules", "Orpheus rules"),
    ]);
    bank.questions.extend([
        short("sources-converter", "sources", SOURCES,
            "What device converts an analog recording into a digital file?",
            "An analog-to-digital converter, such as a sound card.",
            &["Analog-to-digital converter"], &["ADC", "Analog to digital converter", "A sound card", "Sound card"]),
        explain("sources-media", "sources", SOURCES,
            "How do cassette tape and vinyl store sound, and how are they copied to digital files?",
            "Tape stores changes in magnetization; vinyl stores grooves. Play the medium into a recording device or sound card, using a preamp for vinyl.",
            &["Tape stores magnetization changes; vinyl stores physical grooves.", "Playback is captured through an analog-to-digital recording device; vinyl uses a preamp."], &[],
            &["Vinyl already stores computer-readable digital files."]),
        explain("sources-approval", "sources", SOURCES,
            "Which analog sources need moderator approval before upload, and which does the guide always approve?",
            "Vinyl is approved. Cassette tapes and other analog sources require moderator approval.",
            &["Vinyl is approved.", "Cassette and other analog media require moderator approval."], &[],
            &["All analog sources can be uploaded without approval."]),
        explain("sources-digital", "sources", SOURCES,
            "What distinguishes a digital music source? Give two examples and state the check needed before upload.",
            "Digital sources already store computer-readable binary data. Examples include CDs, DVDs, SACDs and WEB downloads. Check for lossy transcodes using spectral analysis.",
            &["Digital media already store computer-readable data.", "Give two of CD, DVD, SACD or WEB download.", "Check for lossy transcodes with spectral analysis."], &[],
            &["A digital source guarantees that its audio is lossless."]),
        explain("formats-categories", "formats", FORMATS,
            "Classify WAV/AIFF, FLAC/ALAC/APE, and MP3/AAC/WMA/AC3/DTS into the guide's three format categories.",
            "WAV and AIFF are uncompressed lossless PCM. FLAC, ALAC and APE are compressed lossless. MP3, AAC, WMA, AC3 and DTS are lossy.",
            &["WAV and AIFF are uncompressed lossless.", "FLAC, ALAC and APE are compressed lossless.", "MP3, AAC, WMA, AC3 and DTS are lossy."], &["PCM means pulse-code modulation."],
            &["FLAC is lossy."]),
        short("formats-bitrate-unit", "formats", FORMATS,
            "In what unit does the guide express audio bitrate?",
            "Kilobits per second, abbreviated kbps.", &["Kilobits per second"], &["kbps", "kbit/s"]),
        explain("formats-bitrate-definition", "formats", FORMATS,
            "What does audio bitrate measure, and what does 320 kbps mean?",
            "Bitrate measures the number of bits conveyed per unit of time. At 320 kbps, 320 kilobits are conveyed per second.",
            &["Bitrate measures bits conveyed per unit of time, so 320 kbps means 320 kilobits per second."], &[],
            &["Bitrate measures audio frequency or song duration."]),
        explain("formats-size", "formats", FORMATS,
            "For the same recording, why do uncompressed lossless, compressed lossless and lossy files use different amounts of space?",
            "Uncompressed lossless gives silence the same bits per second as sound. Compressed lossless encodes the audio more compactly, using very few bits for silence. Lossy encoding saves space by discarding some original audio data.",
            &["Uncompressed lossless uses the same bits per second for silence and sound.", "Compressed lossless represents the same audio more compactly, such as using very few bits for silence.", "Lossy encoding saves space by discarding part of the recorded data."], &["Lossy removal often affects higher frequencies."],
            &["Smaller always means worse quality, including lossless compression."]),
        explain("formats-allowed-lossy", "formats", FORMATS,
            "Which lossy formats does the guide allow, and what minimum CBR MP3 bitrate does it give?",
            "MP3, AAC, AC3 and DTS are allowed; CBR MP3 must be at least 192 kbps. AAC has special trump rules, with protection for iTunes Exclusive tracks.",
            &["MP3, AAC, AC3 and DTS are allowed.", "The minimum CBR MP3 bitrate is 192 kbps."], &["AAC has an iTunes Exclusive tracks exception to MP3 trumping."],
            &["WMA is an allowed upload format."]),
        short("mp3-v0", "mp3", MP3,
            "Which LAME VBR preset has the highest quality?", "V0 is highest; V9 is lowest.",
            &["V0"], &["-V0", "V 0"]),
        short("mp3-lame-name", "mp3", MP3,
            "What does LAME stand for in the guide?", "LAME Ain't An MP3 Encoder.",
            &["LAME Ain't An MP3 Encoder"], &["LAME Ain’t An MP3 Encoder"]),
        explain("mp3-presets", "mp3", MP3,
            "What are V0 and V2's target bitrates, and when is V2 allowed for music torrents?",
            "V0 targets about 245 kbps; V2 targets about 190 kbps. Actual averages may differ. V2 is allowed only when no better bitrate exists.",
            &["V0 targets about 245 kbps and V2 about 190 kbps.", "V2 is allowed only if no better bitrate exists."], &[],
            &["Every V0 encode has exactly 245 kbps average bitrate."]),
        explain("mp3-abr", "mp3", MP3,
            "How does ABR differ from CBR and VBR, and why does the guide not recommend it?",
            "ABR varies bitrate while targeting a chosen average, so file size is predictable. CBR keeps one bitrate; VBR targets quality. The guide says ABR's compromise performs neither approach well.",
            &["ABR varies bitrate while targeting a preset average.", "CBR holds bitrate constant; VBR targets quality.", "The guide does not recommend ABR because its compromise performs neither approach well."], &["ABR's target average makes file size predictable."],
            &["ABR and VBR both guarantee the selected average bitrate."]),
        explain("transcodes-chain", "transcodes", TRANSCODES,
            "Compare WAV → FLAC → MP3 with WAV → MP3 → AAC. Which is good, and why?",
            "The first chain is good because lossless conversions precede a single final lossy conversion. The second is bad because it re-encodes an already lossy source.",
            &["WAV to FLAC to MP3 is good; lossy conversion occurs only at the last step.", "WAV to MP3 to AAC is bad because AAC is made from a lossy source."], &[],
            &["Changing the lossy output format makes another lossy conversion acceptable."]),
        short("torrenting-ratio", "torrenting", TORRENTING,
            "You upload 30 GB and download 15 GB. What is your ratio?", "2.0, because uploaded data is divided by downloaded data.",
            &["2"], &["2.0", "2:1"]),
        explain("torrenting-ratio-balance", "torrenting", TORRENTING,
            "Why does the guide call 1.0 a healthy ratio, and what concerns does it describe for ratios far below or above 1.0?",
            "At 1.0, uploaded and downloaded amounts are equal. Far below 1.0 means taking more than you contribute; far above can mean hoarding upload instead of downloading and supporting other peers.",
            &["1.0 means equal uploaded and downloaded data.", "Far below means contributing less than you take.", "Far above can mean hoarding upload rather than downloading."], &[],
            &["A high ratio automatically breaks a golden rule."]),
        explain("torrenting-peers", "torrenting", TORRENTING,
            "What do a torrent file and a client do, and how do peers, leeching and seeding relate?",
            "The torrent file describes the wanted data; a client connects peers sharing it. Leeching downloads pieces and seeding offers them. A peer can upload completed pieces while still downloading.",
            &["A torrent file describes data and a client connects peers sharing it.", "Leeching downloads; seeding offers data to peers."], &["A peer can upload completed pieces while downloading other pieces."],
            &["Seeding cannot begin until every piece is downloaded."]),
        explain("torrenting-credit", "torrenting", TORRENTING,
            "Does seeding automatically earn upload credit? What required-ratio benefit does the guide give for seeding every downloaded torrent at least 72 hours per week, and should you stop then?",
            "Credit requires actual transfer to another peer. Seeding affects required ratio, along with downloaded volume. Seeding every downloaded torrent at least 72 hours weekly gives a required ratio of zero until downloads exceed 20 GB. Keep seeding as long as possible; 72 hours is not a stop instruction.",
            &["Upload credit requires actual data transfer to another peer.", "The required ratio is zero until total downloads exceed 20 GB.", "Continue seeding as long as possible."], &[],
            &["Time spent seeding alone earns upload bytes."]),
        explain("torrenting-watch", "torrenting", TORRENTING,
            "What happens below required ratio, and how can you recover downloading privileges? Include the ratio-watch download limit.",
            "Ratio watch lasts two weeks. Failure to recover disables downloading; downloading over 10 GB during watch disables it immediately. Continue seeding, fill requests or upload allowed new torrents to improve ratio.",
            &["Ratio watch gives two weeks to recover before downloads are disabled.", "Over 10 GB downloaded on watch disables downloads immediately.", "Recover by seeding, filling requests or uploading rule-compliant new torrents."], &[],
            &["Duplicates or bad transcodes are acceptable to repair ratio."]),
        explain("torrenting-leech-types", "torrenting", TORRENTING,
            "How do freeleech and neutral leech affect recorded upload and download?",
            "Freeleech counts upload but not download. Neutral leech counts neither.",
            &["Freeleech excludes download but counts upload.", "Neutral leech excludes both upload and download."], &[],
            &["Neutral leech counts upload toward ratio."]),
        explain("torrenting-connectability", "torrenting", TORRENTING,
            "Why can port forwarding help a BitTorrent client behind NAT?",
            "NAT can block incoming connections. Forward the client's port to the computer running it so outside peers can connect, increasing opportunities to upload.",
            &["Port forwarding routes incoming peer connections through NAT to the computer running the client, increasing opportunities to upload."], &[],
            &["Port forwarding guarantees upload credit without data transfer."]),
        explain("torrenting-private", "torrenting", TORRENTING,
            "How should DHT and the private-torrent flag be configured for Orpheus, and why?",
            "Disable DHT and mark created torrents private. DHT discovers peers without the private tracker; these settings help keep nonmembers out of the peer list.",
            &["Disable DHT.", "Mark uploaded torrents private.", "These settings exclude discovery of nonmember peers outside the tracker."], &[],
            &["Enable DHT to share Orpheus torrents publicly."]),
        explain("torrenting-partial", "torrenting", TORRENTING,
            "Why can a large torrent show many leechers without being a good way to gain ratio?",
            "Some peers are partial seeders. They have only part of the torrent and do not intend to download the rest, so they look like leechers without providing the expected demand.",
            &["Partial seeders appear as leechers because they hold only part of the torrent, but they do not intend to download the rest."], &[],
            &["Every listed leecher intends to download the whole torrent."]),
        explain("torrenting-whitelist", "torrenting", TORRENTING,
            "What does a whitelisted client version written as 4.6.x mean, and why check the whitelist?",
            "The x accepts any number in that position, such as 4.6.0 or 4.6.2. Only approved client versions can connect to Orpheus peers.",
            &["x is a wildcard for any number in that position.", "Only approved client versions can connect to Orpheus peers."], &[],
            &["Approval of one version means all future versions are approved."]),
        short("cd-ripping-program", "cd", CD,
            "Which CD-ripping program does the guide recommend for macOS?", "XLD. EAC is recommended for Windows or Linux with Wine.",
            &["XLD"], &["X Lossless Decoder"]),
        explain("cd-burn-rip", "cd", CD,
            "What is the difference between CD ripping and CD burning, and which ripping tools does the guide suggest?",
            "Ripping extracts music from a CD; burning makes a CD from files. Use EAC on Windows or Linux with Wine, or XLD on macOS, for quality rips and sufficient logs.",
            &["Ripping extracts music; burning writes files to a CD.", "EAC is suggested for Windows/Linux with Wine, XLD for macOS."], &[],
            &["Burning a lossy file restores the original recording."]),
        explain("cd-log-integrity", "cd", CD,
            "What does a ripping log record, and may you edit it to remove an error?",
            "It records the ripping process and errors. Never modify a log, including to remove an error; doing so violates the rules.",
            &["A log records the ripping process and any errors.", "Editing a log for any reason is prohibited."], &[],
            &["Logs may be cleaned up before upload."]),
        explain("cd-cue-role", "cd", CD,
            "What does a cue sheet provide when burning a copy of a CD?",
            "It is a text catalog or table of contents describing the CD layout, allowing a CD identical to the original to be burned.",
            &["A cue supplies the CD layout information needed to recreate the original CD."], &[],
            &["A cue contains the audio samples themselves."]),
        short("dupes-trumping", "dupes", DUPES,
            "What is replacing a torrent that fails the rules with one that follows them called?", "Trumping.",
            &["Trumping"], &["A trump", "Trump"]),
        explain("dupes-editions", "dupes", DUPES,
            "Do a different catalog number and packaging alone make a distinct edition? What evidence matters?",
            "No. Releases may differ without meaningfully different audio. Compare tracklists, order, log TOC, peak levels and pre-gaps. Bonus tracks can make a distinct edition.",
            &["Catalog or packaging differences alone do not establish an edition.", "Meaningful audio differences matter; compare tracklist/order and log TOC, peaks or pre-gaps."], &["Bonus tracks may establish different content."],
            &["Every different pressing is allowed to coexist."]),
        explain("dupes-same-quality", "dupes", DUPES,
            "When is a torrent a dupe, and does a scene label make an otherwise identical torrent different?",
            "Same edition, format, bitrate and comparable sampling rate means a dupe. Scene versus non-scene does not make otherwise identical torrents distinct.",
            &["Same edition and same quality/format/bitrate with comparable sampling rate is a dupe.", "Scene status alone does not avoid duplication."], &[],
            &["Scene and non-scene copies always coexist."]),
        explain("dupes-inactive", "dupes", DUPES,
            "What can happen to a torrent unseeded for two weeks, and what should you do if you have its original files?",
            "It can be trumped by an identical reseed or new rip/encode. If you have the original files, reseed the original torrent instead of uploading a new one.",
            &["A torrent unseeded for two weeks can be trumped.", "Reseed its original torrent if you have the files."], &[],
            &["A briefly unseeded torrent automatically permits a duplicate upload."]),
        explain("dupes-lossy-hierarchy", "dupes", DUPES,
            "How do higher CBR/ABR bitrates, MP3 320 CBR and V0 affect lossy trumps? Can 320 CBR and V0 coexist?",
            "Higher CBR replaces lower CBR; higher ABR replaces lower ABR. 320 CBR or V0 trumps CBR below 320. Both 320 CBR and V0 may coexist if neither is a duplicate.",
            &["Higher CBR trumps lower CBR; higher ABR trumps lower ABR.", "320 CBR and V0 trump lower CBR and may coexist."], &["The guide allows a new lossy format averaging at least 192 kbps when absent."],
            &["V0 always trumps 320 CBR."]),
        explain("dupes-vbr-hierarchy", "dupes", DUPES,
            "In the guide's trump chart, which current VBR presets replace MP3 APS and APX, and can V0 replace V1?",
            "V2 replaces APS; V0 replaces APX and V1. V2 and V0 are separate protected endpoints in the chart, subject to the guide's upload restrictions.",
            &["V2 replaces APS.", "V0 replaces APX and V1."], &[],
            &["APS and APX can never be trumped."]),
        explain("dupes-aac-extras", "dupes", DUPES,
            "When can MP3 trump AAC, and do log/cue files or artwork let a lossy torrent trump an equivalent one?",
            "Allowed MP3 can trump AAC of the same edition and media, except iTunes purchases with iTunes Exclusive tracks. Extra logs, cues, playlists or artwork do not trump an equivalent lossy torrent.",
            &["Allowed MP3 trumps equivalent AAC except the iTunes Exclusive tracks exception.", "Extra files do not trump equivalent lossy audio."], &[],
            &["Adding a cue always permits a lossy trump."]),
        explain("dupes-cd-source", "dupes", DUPES,
            "Can a rip from a CD-R copy of a commercial pressed CD replace the original CD rip? State the CD-R exception.",
            "No. Rips must come from commercially pressed or official artist/label-approved CDs. The exception is a release distributed only on CD-R by the artist or label.",
            &["A CD-R copy of a pressed CD is not an allowed source.", "Artist/label releases distributed only on CD-R are an exception."], &[],
            &["Burning and re-ripping a pressed CD is an equivalent approved source."]),
        explain("dupes-log-scores", "dupes", DUPES,
            "Compare an approved-tool log versus no log, a 100% log versus 80%, and a 99% log versus 80%. Which can trump?",
            "An EAC/XLD log of any score can trump no log or a nonapproved-tool log. A 100% log can trump 80%; 99% cannot trump an existing 80% approved-tool log.",
            &["An approved-tool log can trump no log.", "A 100% log can trump an 80% log.", "A 99% log cannot trump an 80% approved-tool log."], &["An approved-tool log can also trump a nonapproved-tool log."],
            &["Any higher log score can trump a lower one."]),
        explain("dupes-cue-trump", "dupes", DUPES,
            "When can a 100% log rip with a cue trump a 100% log rip without one?",
            "The noncompliant cue must materially differ from a cue generated from the log, such as containing correct extra indices, detected pre-gap lengths or pre-emphasis flags.",
            &["The cue must materially differ from one generated from the log."], &["Material differences can include extra indices, detected pre-gaps or pre-emphasis flags."],
            &["Adding any generated cue is enough to trump."]),
        explain("dupes-tags", "dupes", DUPES,
            "Which tags are required, and what distinguishes a tag trump from a minor spelling fix?",
            "Title, album, artist and track number are required. Missing or completely wrong fields can be trumped with substantial corrections; tiny spelling or diacritic fixes are insufficient. Album capitalization must be proper.",
            &["Title, album, artist and track number are required.", "A tag trump requires substantial correction, not tiny spelling or diacritic changes."], &["Album titles must use proper capitalization."],
            &["Any typo permits a tag trump."]),
        explain("dupes-folders", "dupes", DUPES,
            "What folder and filename rules should a music upload follow, including a single track and a multi-disc release?",
            "Put music in a meaningfully named folder containing at least the album name, even for one track. Avoid archives and needless nesting; filenames must reflect titles. Separate discs into folders if track numbers repeat.",
            &["Use an album-named folder even for a single track.", "No archives or unnecessary nesting; filenames reflect track titles.", "Do not put repeated track numbers from multiple discs in one directory."], &["Year and format in the folder name are encouraged."],
            &["A ZIP archive is an acceptable music folder."]),
    ]);
    spectral_questions(bank);
    cd_exercises(bank);
    rules_questions(bank);
}

fn topic(id: &str, name: &str) -> TopicDto {
    TopicDto {
        topic_id: id.into(),
        name: name.into(),
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).into()).collect()
}

fn short(
    id: &str,
    topic: &str,
    source: &str,
    prompt: &str,
    reference: &str,
    accepted: &[&str],
    aliases: &[&str],
) -> QuestionDto {
    QuestionDto {
        question_id: id.into(),
        topic_id: topic.into(),
        prompt: prompt.into(),
        reference_answer: reference.into(),
        source_url: source.into(),
        material: vec![],
        grading: AnswerCriteria::ShortAnswer {
            accepted_answers: strings(accepted),
            aliases: strings(aliases),
        },
    }
}

#[allow(clippy::too_many_arguments)]
fn explain(
    id: &str,
    topic: &str,
    source: &str,
    prompt: &str,
    reference: &str,
    required: &[&str],
    optional: &[&str],
    contradictions: &[&str],
) -> QuestionDto {
    QuestionDto {
        question_id: id.into(),
        topic_id: topic.into(),
        prompt: prompt.into(),
        reference_answer: reference.into(),
        source_url: source.into(),
        material: vec![],
        grading: AnswerCriteria::Explanation {
            required_points: strings(required),
            optional_points: strings(optional),
            contradictions: strings(contradictions),
        },
    }
}

fn spectral_questions(bank: &mut QuestionBank) {
    let mut full = explain(
        "spectral-lossless",
        "spectral",
        SPECTRALS,
        "What do this diagram's axes represent, how high does the signal extend, and would lossless-to-lossless conversion change it? Compare its upper frequency with the guide's human hearing range.",
        "Time is horizontal and frequency is vertical, measured in Hz/kHz. The signal extends to about 22 kHz, above the guide's approximate 20 Hz to 20 kHz hearing range. Lossless conversions retain the data, so the spectral looks the same.",
        &[
            "Horizontal is time; vertical is frequency in Hz/kHz.",
            "The signal extends to about 22 kHz.",
            "Lossless-to-lossless conversion retains this spectral.",
            "This upper frequency exceeds the approximate 20 kHz upper limit of human hearing.",
        ],
        &[],
        &["Lossless conversion removes high-frequency data."],
    );
    full.material.push(spectral(
        "spectrum-a.png",
        "Spectral diagram A with time and frequency axes.",
    ));
    bank.questions.push(full);
    let classical = explain(
        "spectral-classical",
        "spectral",
        SPECTRALS,
        "Why can a lossless classical piano recording have a sparse spectrum? What high-frequency evidence does the guide suggest checking before calling it a lossy transcode?",
        "A piano recording can naturally contain little high-frequency musical energy. Genre affects appearance, so sparsity alone does not establish a transcode. Look for faint noise reaching about 22 kHz, as in the guide's classical example, rather than assuming every lossless recording has strong energy everywhere.",
        &[
            "Piano music can naturally contain little high-frequency musical energy.",
            "Check for faint high-frequency noise, reaching about 22 kHz in the guide's example.",
        ],
        &[],
        &["All lossless music must have strong energy across every frequency."],
    );
    bank.questions.push(classical);
    for (id, file, cutoff, guide_cutoff, encoding, label) in [
        (
            "spectral-320",
            "spectrum-c.png",
            "20 kHz",
            "20.5 kHz",
            "MP3 320 kbps CBR",
            "C",
        ),
        (
            "spectral-256",
            "spectrum-d.png",
            "19.5 kHz",
            "20 kHz",
            "MP3 256 kbps CBR",
            "D",
        ),
        (
            "spectral-v0",
            "spectrum-e.png",
            "21 to 22 kHz, with much weaker energy above 19 to 20 kHz",
            "19.5 kHz",
            "MP3 V0",
            "E",
        ),
        (
            "spectral-192",
            "spectrum-f.png",
            "18.5 to 19 kHz",
            "19 kHz",
            "MP3 192 kbps CBR",
            "F",
        ),
        (
            "spectral-v2",
            "spectrum-g.png",
            "18.5 kHz",
            "18.5 kHz",
            "MP3 V2",
            "G",
        ),
        (
            "spectral-128",
            "spectrum-h.png",
            "16 to 17 kHz",
            "16 kHz",
            "MP3 128 kbps CBR",
            "H",
        ),
    ] {
        let observation = format!("Visible energy reaches approximately {cutoff}.");
        let comparison = format!("The guide illustrates {guide_cutoff} for {encoding}.");
        let mut question = explain(
            id,
            "spectral",
            SPECTRALS,
            &format!(
                "Estimate the highest visible frequencies in this diagram and compare them with the guide's {encoding} example. Can the cutoff alone identify an exact bitrate or preset? What other common MP3 feature does the guide describe?"
            ),
            &format!(
                "Visible energy reaches about {cutoff}. This image was generated from a real {encoding} encode using LAME's defaults. The guide illustrates a cutoff of {guide_cutoff} for that setting; encoder versions and settings can differ. A cutoff alone cannot identify the exact bitrate or preset. The guide also describes a common MP3 shelf at about 16 kHz."
            ),
            &[
                &observation,
                &comparison,
                "A cutoff alone cannot identify the exact bitrate or preset.",
                "The guide describes an MP3 shelf at about 16 kHz.",
            ],
            &[],
            &["A cutoff uniquely proves the exact MP3 bitrate or preset."],
        );
        question.material.push(spectral(
            file,
            &format!("Spectral diagram {label} with time and frequency axes."),
        ));
        bank.questions.push(question);
    }
    let mut transcode = explain(
        "spectral-disguised-flac",
        "spectral",
        SPECTRALS,
        "This file claims to be FLAC. What does its spectrum suggest, and how should you compare the claimed encoding with the actual spectrum?",
        "The cutoff near 18.5 to 19 kHz is consistent with a lossy source despite the FLAC encoding. This example was made by converting MP3 192 CBR to FLAC, retaining the MP3's missing high frequencies. A cutoff alone cannot prove the source history or exact bitrate. FLAC's extension, size and sound do not prove its origin. Check claimed encoding with a metadata tool, then inspect the spectrum in Audacity, Audition or SoX.",
        &[
            "The cutoff near 18.5 to 19 kHz suggests a possible lossy source; it does not prove source history.",
            "Use metadata to check the claim and spectral software to inspect actual frequency content.",
        ],
        &[
            "FLAC encoding does not restore frequencies discarded by a lossy source.",
            "AudioIdentifier alone is not a reliable transcode detector.",
        ],
        &["A FLAC extension or large file size proves a lossless source."],
    );
    transcode.material.push(spectral(
        "spectrum-i.png",
        "Spectral diagram I for the supplied file, with time and frequency axes.",
    ));
    bank.questions.push(transcode);
}

fn spectral(file: &str, description: &str) -> ExerciseMaterialDto {
    ExerciseMaterialDto::Image {
        src: format!("/exercises/{file}"),
        description: description.into(),
    }
}

fn cd_exercises(bank: &mut QuestionBank) {
    let mut log = explain(
        "cd-log-reading",
        "cd",
        "https://interview.orpheus.network/log-file.php",
        "From this log excerpt, identify the read mode, whether C2 pointers were used, and whether the two CRC values match. Does this excerpt give a whole-log checker score?",
        "Secure read mode; C2 pointers were not used. Test and copy CRCs match. The excerpt's track quality is not a whole-log checker score, which is not supplied here.",
        &[
            "Read mode is Secure and C2 pointers are disabled.",
            "Test and copy CRCs match.",
            "The excerpt does not supply a whole-log checker score.",
        ],
        &[],
        &["Track quality 100.0% guarantees a 100% whole-log checker score."],
    );
    log.material.push(ExerciseMaterialDto::Excerpt {
        description: "Selected settings and track fields from the official EAC sample log; omitted lines are not shown.".into(),
        text: "Read mode               : Secure\nMake use of C2 pointers : No\n\nTrack  1\n     Track quality 100.0 %\n     Test CRC 02B2C5A6\n     Copy CRC 02B2C5A6".into(),
    });
    bank.questions.push(log);
    let mut cue = explain(
        "cd-cue-reading",
        "cd",
        "https://interview.orpheus.network/cue-file.php",
        "Which audio track does this cue excerpt describe? Identify its pre-gap and index position, and explain why the cue is useful for burning.",
        "It describes audio track 01, with PREGAP 00:00:32 and INDEX 01 at 00:00:00. The cue supplies CD layout information for recreating the original; it does not contain audio.",
        &[
            "The excerpt describes AUDIO track 01.",
            "PREGAP is 00:00:32 and INDEX 01 is 00:00:00.",
            "A cue provides CD layout information for recreating the original CD.",
        ],
        &[],
        &["The cue contains the audio samples themselves."],
    );
    cue.material.push(ExerciseMaterialDto::Excerpt {
        description: "Selected fields from the official sample cue; omitted lines are not shown."
            .into(),
        text: "  TRACK 01 AUDIO\n    PREGAP 00:00:32\n    INDEX 01 00:00:00".into(),
    });
    bank.questions.push(cue);
}

fn rules_questions(bank: &mut QuestionBank) {
    bank.questions.extend([
        short("rules-disabled", "rules", RULES,
            "Which IRC channel should a person with a disabled Orpheus account use?", "#disabled. Do not create another account or interview again.",
            &["#disabled"], &["disabled"]),
        explain("rules-account", "rules", RULES,
            "What are Orpheus's lifetime-account, account-transfer, sharing and activity rules?",
            "One personal account per lifetime. Never trade, sell, give away or share it. Log in regularly. Ask staff to disable an unwanted account.",
            &["One account per person per lifetime.", "No transfer or sharing of accounts.", "Log in regularly."], &["Ask staff to disable an unwanted account."],
            &["A disabled account permits a new account."]),
        explain("rules-invitees", "rules", RULES,
            "Which invitee behavior can put your account at risk, and what invitee failure does the guide exempt?",
            "Invite only people you know and trust. Invitee golden-rule violations threaten your invite privileges and account; failing required ratio alone is exempt.",
            &["Invitees' golden-rule breaches can endanger the inviter's privileges or account.", "Invitees' ratio failure alone is not punished against the inviter."], &["Invite only people you know and trust."],
            &["Inviters are never responsible for invitees."]),
        explain("rules-invites", "rules", RULES,
            "What restrictions apply to offering and requesting invites, including the forum and recruiter exceptions?",
            "Do not sell, trade or publicly offer invites, except staff-designated recruiters in approved places. Do not request invites/accounts or send unsolicited requests. Eligible users may respond privately to an offer in the Invites Forum.",
            &["No trading, selling or public offers, except designated recruiters in approved locations.", "No invite/account requests or unsolicited messages.", "Messaging in response to an Invites Forum offer is permitted."], &["The Invites Forum is restricted to Elite and above."],
            &["Private unsolicited requests are allowed."]),
        explain("rules-ratio", "rules", RULES,
            "Why are buffer transfers through unintended site features and false tracker reports prohibited?",
            "Unintended buffer transfer or inflation is ratio manipulation. False tracker reports are cheating, even with an approved client. Ask staff if unsure.",
            &["Unintended buffer transfer/increase is ratio manipulation.", "Reporting false data is cheating even with an approved client."], &["Ask staff about uncertainty."],
            &["An approved client makes manipulated reports acceptable."]),
        explain("rules-client", "rules", RULES,
            "What restrictions apply to modified clients, client access and testing unstable clients?",
            "Use an unmodified whitelisted client with direct, exclusive access. Developers need staff approval before testing unstable clients.",
            &["Use only unmodified whitelisted clients with direct exclusive access.", "Unstable-client testing requires prior staff approval."], &[],
            &["A modified approved client is automatically approved."]),
        explain("rules-torrent-passkey", "rules", RULES,
            "Why must you neither share Orpheus torrent files nor add other announce URLs to them?",
            "Orpheus torrent announce URLs contain your personal passkey. Sharing exposes it. Adding other announce URLs is prohibited and can cause false tracker reporting interpreted as cheating.",
            &["Sharing torrent files exposes your personal passkey.", "Adding outside announce URLs can cause false tracker reporting interpreted as cheating."], &[],
            &["The torrent file is safe to share if your password is withheld."]),
        explain("rules-privacy", "rules", RULES,
            "What restrictions protect other users' private information, including investigations and automated collection?",
            "Do not expose, threaten or blackmail users with private information. Do not share unvolunteered information without permission or harvest identifying data with scripts/exploits, including information found by investigating public details.",
            &["Do not disclose unvolunteered private information without permission or use it for threats or blackmail.", "Do not harvest identifying data with scripts or exploits.", "Privacy protection also covers information found by investigating public details."], &[],
            &["Information found through a search may always be published."]),
        explain("rules-conduct", "rules", RULES,
            "How should you respond to a suspected rule breach? Include the rules about scams and impersonating staff.",
            "Report suspected breaches rather than confronting or policing users. Scams and phishing are prohibited, as is impersonating staff or official service accounts, on-site, off-site and on IRC.",
            &["Report suspected breaches rather than backseat moderating.", "No scams/phishing.", "No staff/service impersonation on-site, off-site or IRC."], &[],
            &["Confronting a suspected rule breaker is the preferred response."]),
        explain("rules-staff", "rules", RULES,
            "How should you challenge a staff decision or seek clarification of a rule?",
            "Discuss privately with the deciding moderator, or Staff PM if unavailable. Do not shop around moderators; an administrator can give a second opinion. Respect volunteers and accept staff's final rule interpretation; ask via Staff PM for clarification.",
            &["Challenge a decision privately with the deciding moderator, or Staff PM if unavailable; seek rule clarification via Staff PM.", "Do not shop around moderators for a favorable answer; an administrator may give a second opinion."], &["Respect staff and their final rule interpretation."],
            &["User consensus overrides staff's interpretation."]),
        explain("rules-events-commercial", "rules", RULES,
            "May users request special events or use Orpheus services for commercial gain?",
            "No. Staff choose special events without a fixed schedule. Commercializing services or user data, referral schemes, solicitations and money offers are prohibited.",
            &["Special events are staff's decision and cannot be requested.", "Commercial gain, referral schemes, solicitations and money offers are prohibited."], &[],
            &["Freeleech must happen on a fixed schedule."]),
        explain("rules-vpn", "rules", RULES,
            "Which VPN/proxy services may be used to browse Orpheus, and is Tor permitted?",
            "Use paid services, including self-hosted services or a seedbox's VPN/proxy. Free services and Tor browsing are prohibited. Ask staff when uncertain.",
            &["Use only paid, self-hosted or seedbox VPN/proxy services; free services are prohibited.", "Tor browsing is prohibited."], &["Ask staff when uncertain."],
            &["Any free VPN is allowed."]),
        explain("rules-automation", "rules", RULES,
            "How must automated site access work, at what rate, and may it autosnatch freeleech?",
            "Use the API, at most five requests in any ten-second window. Do not scrape site HTML or automatically snatch freeleech with little or no user input.",
            &["Automated access uses the API, not HTML scraping.", "At most five requests within any ten-second window.", "No freeleech autosnatching with little or no user input."], &[],
            &["Staying under the rate limit permits HTML scraping or freeleech autosnatching."]),
        explain("rules-bugs", "rules", RULES,
            "What should you do about a live-site bug, and what restrictions apply to exploits?",
            "Do not seek or exploit live bugs. Immediately report critical vulnerabilities under the Responsible Disclosure Policy; report noncritical bugs in the Bugs Forum. Do not publish, share or facilitate exploits; staff decide their classification.",
            &["Do not seek or exploit live bugs.", "Report critical bugs through responsible disclosure and noncritical bugs in the Bugs Forum.", "Do not publish, share or facilitate exploits."], &["Staff decide exploit classifications."],
            &["Testing a live vulnerability is allowed before reporting it."]),
    ]);
}

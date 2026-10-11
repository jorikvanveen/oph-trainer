# Spectrogram question audit

Reviewed on 2026-10-11 against the guide's [spectral analysis](https://interview.orpheus.network/spectral-analysis.php), [audio formats](https://interview.orpheus.network/audio-formats.php) and [transcodes](https://interview.orpheus.network/transcodes.php) pages, including the bundled exercise diagrams. The 14 remaining questions require guide knowledge and direct observations of the supplied images.

| Question ID | Basis for the required answer |
| --- | --- |
| `spectral-axes` | Spectral overview and diagram A's axis labels. |
| `spectral-lossless` | Spectral overview's hearing range, CD / Lossless section and diagram A. |
| `spectral-lossless-conversion` | CD / Lossless section's unchanged spectrum after lossless conversion. |
| `spectral-classical` | CD / Lossless section's piano example, reproduced as diagram B. |
| `spectral-piano-noise` | Apply the piano example to diagram K's sparse music and upper-band noise. |
| `spectral-piano-transcode` | Audio formats' discarded lossy data and transcodes' lossy-to-lossless conversion; the prompt supplies the encoding history. |
| `spectral-320` | Read diagram C's tallest purple spike tips and compare with the supplied MP3 reference. |
| `spectral-256` | Read diagram D's tallest purple spike tips and compare with the supplied MP3 reference. |
| `spectral-192` | Read diagram F's tallest purple spike tips and compare with the supplied MP3 reference. |
| `spectral-v2` | Read diagram G's tallest purple spike tips and compare with the supplied MP3 reference. |
| `spectral-128` | Read diagram H's tallest purple spike tips and compare with the supplied MP3 reference. |
| `spectral-v0` | Read diagram E's faint streaks and compare with the supplied MP3 reference. |
| `spectral-mp3-shelf` | MP3 section's stated shelf frequency. |
| `spectral-disguised-flac` | Transcodes section's FLAC example and diagram I's cutoff. |

Removed `spectral-piano-display-range` and `spectral-cutoff-limits`. The guide does not teach display floors or how encoder versions and settings affect cutoffs. Removed grading requirements about proving source history and exact encoding from the other questions. The shelf question now asks only for the frequency stated in the guide.

The generated images differ from the guide's illustrations. Their comparison questions ask learners to read the images and compare their observations with the supplied references. Estimate ranges remain explicit in the grading criteria. No answer requires dBFS, rendering settings or encoder behavior.

The five spike comparison prompts previously asked for the top edge of the "purple band". Diagram F's continuous band ends near 16 kHz, while its tallest purple spikes reach about 19 kHz. The prompts and grading now identify the spikes explicitly, and the reference answers distinguish them from the continuous band. Comparison criteria reject "far below" or "far above" when the expected relationship is near the reference.

#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 || $# -gt 2 ]]; then
  echo "Usage: $0 <source.flac> [piano.flac]" >&2
  exit 2
fi

source_audio=$(realpath -- "$1")
repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
output_dir="$repo_root/frontend/static/exercises"
work_dir=$(mktemp -d)
trap 'rm -rf -- "$work_dir"' EXIT

check_source() {
  local audio=$1 sample_rate
  flac -t -- "$audio"
  sample_rate=$(ffprobe -v error -select_streams a:0 -show_entries stream=sample_rate -of default=nw=1:nk=1 "$audio")
  if [[ "$sample_rate" != 44100 ]]; then
    echo 'The authored exercises require a 44.1 kHz source.' >&2
    exit 1
  fi
}

check_source "$source_audio"
if [[ $# -eq 2 ]]; then
  piano_audio=$(realpath -- "$2")
  check_source "$piano_audio"
fi

ffmpeg -v error -ss 45 -i "$source_audio" -t 45 -map 0:a:0 -c:a pcm_s16le "$work_dir/source.wav"

render() {
  local audio=$1 label=$2 gain=${3:-0.5}
  # Decoded MP3 peaks can exceed full scale; attenuate before integer PCM conversion.
  ffmpeg -v error -i "$audio" -af "volume=$gain" -c:a pcm_s24le "$work_dir/render-$label.wav"
  sox "$work_dir/render-$label.wav" -n remix 1 spectrogram \
    -x 3000 -y 1025 -z 130 -Z 0 -w Kaiser \
    -t "Spectral diagram ${label^^}" \
    -c 'Left channel | 44.1 kHz | excerpt 00:45 to 01:30 | analysis gain -6.02 dB' \
    -o "$output_dir/spectrum-$label.png"
}

render "$work_dir/source.wav" a

for setting in c:320 d:256 e:V0 f:192 g:V2 h:128; do
  label=${setting%%:*}
  encoding=${setting#*:}
  if [[ "$encoding" == V* ]]; then
    encoder_args=(-q:a "${encoding#V}")
  else
    encoder_args=(-b:a "${encoding}k")
  fi
  ffmpeg -v error -i "$work_dir/source.wav" -c:a libmp3lame "${encoder_args[@]}" "$work_dir/$label.mp3"
  render "$work_dir/$label.mp3" "$label"
done

ffmpeg -v error -i "$work_dir/f.mp3" -af volume=0.5 -c:a flac -sample_fmt s32 -bits_per_raw_sample 24 "$work_dir/transcode.flac"
render "$work_dir/transcode.flac" i 1

if [[ $# -eq 2 ]]; then
  ffmpeg -v error -ss 45 -i "$piano_audio" -t 45 -map 0:a:0 -c:a pcm_s16le "$work_dir/piano.wav"
  render "$work_dir/piano.wav" k
  ffmpeg -v error -i "$work_dir/piano.wav" -c:a libmp3lame -b:a 128k "$work_dir/piano.mp3"
  ffmpeg -v error -i "$work_dir/piano.mp3" -af volume=0.5 -c:a flac -sample_fmt s32 -bits_per_raw_sample 24 "$work_dir/piano-transcode.flac"
  render "$work_dir/piano-transcode.flac" l 1
fi

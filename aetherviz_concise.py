# aetherviz_concise.py — Pure Just Intonation + MIDI + Sheet
import numpy as np
import wave
import struct
import mido
from mido import MidiFile, MidiTrack, Message, MetaMessage
import hashlib

SAMPLE_RATE = 48000
DURATION = 8.0
BPM = 120
BEATS_PER_SEC = BPM / 60

# Just Intonation ratios from A=110Hz (MIDI base 45)
RATIOS = [1, 9/8, 5/4, 4/3, 3/2, 5/3, 15/8]
CARRIERS = [110.0 * r for r in RATIOS]

# MIDI notes (A2=45 base)
BASE_MIDI = 45
LINE_MIDI = [
    [0, 3, 0],      # A E A
    [3, 7, 5],      # E A F#
    [5, 0, 3, 7],   # F# A E A(oct)
    [0],            # sustained A
    [3, 7],         # E → A gliss
    [5, 1, 4]       # F# B E
]

def adsr(t, dur):
    a,d,s,r = 0.5,0.7,0.75,1.0
    return np.piecewise(t, 
        [t < a, t < a+d, t < dur-r, t < dur],
        [lambda t: t/a,
         lambda t: 1 - (1-s)*(t-a)/d,
         s,
         lambda t: s*(1 - (t - (dur-r))/r),
         0])

def charity_gliss(t):
    return np.where(t < 1.2, np.sin(2*np.pi*(165 + t/1.2*110)*t) * 0.38 * np.exp(-t), 0)

def node137_burst(carrier, t):
    freq = carrier * (1.618**(1/12))
    return np.where(t < 0.8, (2*(freq*t % 1)-1) * np.exp(-t*6) * 0.45, 0)

def harmonics(carrier, t):
    return (np.sin(2*np.pi*(carrier/2)*t)*0.25 + np.sin(2*np.pi*carrier*2*t)*0.15 +
            np.sin(2*np.pi*carrier*2.87*t)*0.06 + np.sin(2*np.pi*carrier*3.41*t)*0.06)

def render(carrier):
    n = int(DURATION * SAMPLE_RATE)
    t = np.linspace(0, DURATION, n, False)
    sig = np.sin(2*np.pi*carrier*t) * 0.9 + harmonics(carrier, t)
    sig += charity_gliss(t) + node137_burst(carrier, t)
    return sig * adsr(t, DURATION)

def write_wav(stereo):
    stereo = np.int16(stereo / np.max(np.abs(stereo)) * 32767)
    with wave.open("aether_invocation.wav", 'w') as wf:
        wf.setnchannels(2)
        wf.setframerate(SAMPLE_RATE)
        wf.setsampwidth(2)
        wf.writeframes(stereo.T.tobytes())

def write_midi():
    mid = MidiFile()
    track = MidiTrack()
    mid.tracks.append(track)
    track.append(MetaMessage('set_tempo', tempo=mido.bpm2tempo(BPM)))
    for notes in LINE_MIDI:
        for note in notes:
            track.append(Message('note_on', note=BASE_MIDI + note, velocity=100, time=0))
        track.append(Message('note_off', note=BASE_MIDI + notes[-1], velocity=0, time=int(4 * 480)))
    mid.save("aether_invocation.mid")

# Render mix
mix = np.zeros((2, int(DURATION * SAMPLE_RATE)))
for i, carrier in enumerate(CARRIERS):
    sig = render(carrier)
    pan = [(0.5,0.5),(0.7,0.3),(0.3,0.7),(0.8,0.2),(0.2,0.8),(0.6,0.4),(0.5,0.5)][i]
    mix[0] += sig * pan[0]
    mix[1] += sig * pan[1]

write_wav(mix)
write_midi()
print("Rendered: aether_invocation.wav + .mid")
print("Hash:", hashlib.sha256(mix.tobytes()).hexdigest())

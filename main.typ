#import "@preview/touying:0.7.3": *
#import themes.simple: *

#import "@preview/codly:1.3.0": *
#import "@preview/codly-languages:0.1.10": *
#show: codly-init.with()
#codly(languages: codly-languages)

#show: simple-theme.with(aspect-ratio: "16-9") 

#show link: lonk => [
  #text(stroke: blue)[
    #lonk
  ]
]

#title-slide[
  = Taran's Plato Project: Digital Signal Processing
  #v(2em)

  Taran Nathan
]

== My Main Question
- What effects do digital signal processing software algorithms have on modern software today?
- Digital signal processing: the manipulation and analysis of digital signals using algorithms and techniques
- Used for: audio processing, telecommunications, medical imaging, etc.

== Why did I pick this project
- I like programming stuff for fun
- It is a really interesting field, and I wanted to learn more about it.
  - I had actually watched a really interesting video on the topic the night before the project was announced, and that is one of the main things that inspired me. 

== What I built
- Two main parts to my project:
  - Informative slide presentation on Digital Signal Processing (DSP)
  - Breath-rate detection system using a camera and AI
- Slideshow kind of got neglected for most of the project :(

== High Level overview of the Breath Detection System
- Camera captures video of a person
- AI model (google blazepose) detects body landmarks (key points on body, face, hands, shoulder, etc.)
- Detected positions are exported to a CSV file
- DSP system analyzes data to estimate breath rate

== Reason to have CSV as middleman in between Detection and Analysis
- Create 1 controlled test sample that I can reuse (recorded breathing)
- Allow me to manually verify (count breaths in sample)

== More benefits to modularizing detection and signal analysis
- Signal analysis can run on my laptop, while my desktop computer can run the AI inference
- Faster iteration on the actual DSP algorithms
  - because I only need to rerun DSP, instead of re-measuring from video and then analyzing


= Timeline

- February: I researched and pulled together resources about DSP  
  - #link("https://www.robots.ox.ac.uk/\~sjrob/Teaching/SP/l7.pdf")
  - ^ about Discrete Fourier Transform, very important  
  - #link("https://www.analog.com/media/en/training-seminars/design-handbooks/MixedSignal\_Sect6.pdf")
  - ^ writeup on types of Digital Signal Filters  

==
- March: Tried to dive deeper, looked into implementing my own Discrete Fourier Transform and Low pass filter  
  - #link("https://www.3blue1brown.com/lessons/fourier-transforms/")My favorite introduction to the fourier transform  
- Planned out what I would need, came up with Project Idea: Human Breath detection and speed calculation

==

- March \- May: Continued working on project  
  - March 22: Slapped together an human pose detection AI model, but no data analysis complete yet  
  #image("images/landmarking.png")
  - March 28 (Most of this Saturday and Sunday Morning): finished up shoddy prototype  
    - Could somewhat accurately detect breath speed \~30% of time with no filters, only Discrete Fourier Transform  
    #image("./images/graphs.png", width: 80%)
    - ^^^ Me trying to graph the data in a spreadsheet to try to help debug my program  
    - Person detection was finicky, required specific orientation relative to camera  
  - April 11 (Most of this weekend)  
    - Added a basic low pass filter that upped accuracy to \~60-65%  
    #image("images/filter.png", width: 30%)
- Current status (finished \~May 10): \~75-85% accuracy, improved flexibility in target human pose due to multiple improvements (improved low and high pass filter, improved Fourier Transform implementation, and better interpreting the outputs of the Fourier Transform)

== 

#image("images/output-cropped.png", width: 90%)
- roughly means I was breathing at ~9.8 breaths per minute (accurate, I was breathing slowly in this sample for testing purposes)

= Learning Time! :)

#title-slide[
= DSP Fundamentals
#v(2em)
(time to get serious 🧐)
]

== What *_is_* a Signal?

- A signal is just data that changes over time
  - Audio waveforms
  - Sensor measurements
  - Radio transmissions
  - Body movement data
- DSP works by mathematically analyzing these changing values
- All _Digital_ Signal Processing is done on Digital signals, not Analog
  - this means quantized chunks of data, not a continuous stream of data

#figure(
  image("images/sine.png", width: 70%),
  caption: [Example of a waveform / signal, a simple sinewave]
)

== Analog vs Digital Signals

#columns(2)[
    *Analog*
    - Continuous
    - Real-world signals
    - Infinite possible values
    - Example: microphone voltage

    #colbreak()

    *Digital*
    - Discrete samples
    - Stored as numbers
    - Easier to process with software
    - Example: WAV audio file
]

- DSP focuses on the *digital* representation of signals

== Sampling

- Computers cannot continuously measure signals
- Instead, they take many tiny measurements per second
- Each measurement is called a *sample*

#figure(
  image("images/sampling.png", width: 75%),
  caption: [Continuous signal being sampled into discrete points]
)

- Higher sample rate = more accurate representation

== Noise

- Real signals are messy
- Cameras, microphones, sensors, and the environment all introduce unwanted variation in our signal
- This unwanted variation is called *noise*

- My breathing data had:
  - pose estimation inaccuracies
  - random movement in the human target (mainly me) and the camera

- DSP filters help isolate the useful signal from the noise

== Filters

- Filters remove or reduce unwanted frequencies

#columns(2)[
    *Low-pass filter*
    - Removes high frequencies, keeps lower trends
    - Useful for breathing data, slow frequencies (<.2 Hz)

    #colbreak()

    *High-pass filter*
    - Removes slow drift
    - Keeps fast changes
]
- My project mainly relied on low-pass filtering #footnote[though near the end I added a high-pass filter for funsies, but it turned to improve accuracy.] 


== Simple Low Pass Filter Example

```python
alpha = 0.1
def low_pass_filter(data: list[float]):
    filtered = []
    previous = data[0]
    for value in data:
        previous = previous + alpha * (value - previous)
        filtered.append(previous)

    return filtered
```

- Every iteration, it adds the scaled down difference between the data and the previous result to the filtered list
- Smooths sudden jumps in data
- Helps stabilize noisy measurements

== Fourier Transform

- One of the most important DSP tools
- Converts a signal from:
  - *time domain* → *frequency domain*

- Instead of asking:
  - "What amplitude(value) does the signal have right now?"

- We ask:
  - "What frequencies exist inside this signal, and what are their amplitudes?"

== What does converting domain mean?
- Any signal can be thought of as a function, and domain is the input to the function.
 $ "function" = "domain" -> "range, ie. output" $
```python
def signal(time):
  return sin(time)
```
- This function takes a time value, and returns an amplitude (this is essentially what sampling is.)
#pagebreak()
- So when we convert to frequency domain, we are changing the input of the function to frequency, not time.
```python
def signal(frequency):
  ...
  return amplitude
```
- This still returns amplitude, but now it is the amplitude of the frequency in the sample, not of the amplitude at a point in time

== Why Fourier Transform Matters

- Fourier analysis helps detect repeating patterns
  - Breathing is periodic
  - Music notes are periodic
  - Radio waves are periodic
- Fourier transforms help us get the strength of any frequency in any of these
  - Therefore, we can get the strongest frequency, and that is what is useful in my project


#figure(
  image("images/fourier.png", width: 75%),
  caption: [Frequency peaks in a transformed signal]
)

== Discrete Fourier Transform (DFT)

- The DFT analyzes digital sampled signals, not a continuous signal #footnote[those are called CFT, Continuous Fourier Transform]
- My project used a Fast Fourier Transform (FFT)
  - optimized implementation of the DFT

- FFT is dramatically faster for large datasets

== Usage in my project

- Peaks in magnitudes reveal dominant frequencies
- strongest frequency ≈ breathing rate
- Due to this, I was roughly able to get the breathing rate of the target

#title-slide[
  = Thanks for listening!
  #v(2em)
  #columns(2)[
    #image("images/thumsup.png", width: 40%)

    #colbreak()

    This presentation is open source, see the code behind this presentation at

    #text(size: 18pt)[#link("https://github.com/moogoesmeow0/english10h-plato-project/blob/main/main.typ")[github.com/moogoesmeow0/english10h-plato-project/blob/main/main.typ]]
  ]
]

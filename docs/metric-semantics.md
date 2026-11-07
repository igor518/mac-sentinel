# Metric Semantics

This document defines the units and identity rules MacSentinel must use. It is a contract for future implementation, not a claim that the metrics are already available.

## CPU

- System CPU is normalized to a range from 0% to 100% across the whole machine.
- Process CPU uses 100% per logical core. A multithreaded process can therefore exceed 100%.
- CPU values require two observations separated by a meaningful sampling interval. The first observation is a baseline and must not be presented as measured activity.
- CPU measurements retain their original precision internally, while user-facing CPU percentages are rounded to the nearest whole percent.

## Memory

- Process memory is resident memory in bytes unless a screen explicitly labels another measure.
- System memory presents used and total physical memory in bytes.
- Display formatting may use human-readable binary units, but stored values remain bytes.

## Process identity

A process is identified by `ProcessKey = PID + start time`. PID alone is never sufficient for history, alerts, or actions because macOS can reuse it after a process exits.

## Sampling continuity

The default sampling cadence is two seconds. A gap longer than twice the configured cadence breaks continuity and resets any sustained-alert timer. Missing or denied data is shown as unavailable, never replaced with zero.

## Alert evidence

The default sustained CPU rule observes a process above 80% of one logical core for ten continuous minutes. Evidence records preserve the process key, observed interval, threshold, measured values, and rule version.

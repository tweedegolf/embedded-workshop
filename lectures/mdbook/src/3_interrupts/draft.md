# What are interrupts
- Automatic handling of hardware events
# What are they good for
- (almost) instantly reacting to events
- power saving
- running tasks concurrently

# How do interrupts work
- What happens from a high level
  - program code paused
  - ISR Run
  - program code contines

## nvic
- core + nvic block diagram
- what does the nvic do

## interrupt call mechanism
  - pause program code
  - push registers to stack
  - look up ISR address in vector table
  - execute ISR
  - pop registers from stack
  - continue program code

## nested interrupts
 - interrupt priority can be dynamically altered
 - Interrupt with higher priority pauses other interrupts


# configuring interrupts using cortex-m-rt
- basic steps
  - configure peripherals for interrupt source 
  - initialize globals used in ISR
  - set interrupt priority
  - unmask interrupt in NVIC

- globals
    - pattern `static GPIOTE_HANDLE: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));`
    - `interrupt_free`
    - `CriticalSection::new`, why is it safe
    - Atomics

# What to take into account
- multiple threads like os, but higher prio pre-empts lower prio
  - avoid non-reentrant function calls
- make ISR as short as possible to avoid missed interrupts
  - use flags to be handled by program (atomics)
  - avoid locks
- don't forget to unpend interrupt

# RTIC (Real Time Interrupt-driven Concurrency) usage
- Application outline
    - init
        - runs with interrupts disabled
        - initialize the peripherals
    - idle
        - what happens when there is nothing to do
        - can be interrupted
        - mostly used to put device to sleep
    - resources
      - Resources initalized with `#[init attribute]`
      - LateResources initialized them in init
      - decide what to for each task

    - tasks
        - hardware tasks: pieces of work that run in reaction to hardware event
        - software task: can be spawned from other contexts
        - dispatched from interrupt handlers (`NVIC::pend`)
          - 'sacrifice' interrupts for software task in `extern` block
          - message passing using task signature
            - capacity task message queue default 1, error if full
          - can be scheduled to run in the future
            - requires declaring a monotonic timer in app attribute
            - takes up space in task message queue
            - uses the SysTick interrupt handler
        - task priorities
         - higher number = higher priority
         - higher priority tasks preempt lower priority



https://microcontrollerslab.com/nested-vectored-interrupt-controller-nvic-arm-cortex-m/

https://microcontrollerslab.com/what-is-interrupt-vector-table/
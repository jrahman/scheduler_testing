#!/bin/bash

U=$USER
sudo bash -c 'echo 1 > /sys/kernel/tracing/events/sched/sched_waking/enable'
sudo bash -c 'echo 1 > /sys/kernel/tracing/events/sched/sched_switch/enable'
sudo bash -c 'echo 1 > /sys/kernel/tracing/events/sched/sched_wakeup_new/enable'
sudo bash -c 'echo 1 > /sys/kernel/tracing/events/task/task_newtask/enable'
sudo bash -c 'echo 1 > /sys/kernel/tracing/events/task/task_rename/enable'

sudo bash -c 'echo > /sys/kernel/tracing/trace'
sudo bash -c 'echo 1 > /sys/kernel/tracing/tracing_on'
sleep 1
sudo bash -c 'cp /sys/kernel/tracing/trace ./trace'
sudo bash -c 'echo 0 > /sys/kernel/tracing/tracing_on'

sudo bash -c 'echo 0 > /sys/kernel/tracing/events/sched/sched_waking/enable'
sudo bash -c 'echo 0 > /sys/kernel/tracing/events/sched/sched_switch/enable'
sudo bash -c 'echo 0 > /sys/kernel/tracing/events/sched/sched_wakeup_new/enable'
sudo bash -c 'echo 0 > /sys/kernel/tracing/events/task/task_newtask/enable'
sudo bash -c 'echo 0 > /sys/kernel/tracing/events/task/task_rename/enable'

sudo chown $U ./trace




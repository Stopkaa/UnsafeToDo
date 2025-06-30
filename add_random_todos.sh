#!/bin/bash

# Anzahl der zufälligen Todos (Standard: 10)
NUM_TODOS=${1:-10}

# Beispiel-Texte
TASKS=("Aufräumen" "Bericht schreiben" "Code review" "Kaffee trinken" "Bug fixen" "Doku lesen" "Meeting vorbereiten" "Backup machen")
PRIORITIES=("low" "medium" "high")
DESCRIPTIONS=("Dringend" "Später erledigen" "Optional" "Mit Deadline" "Teamarbeit" "Debuggen" "Testen")
DATES=("01.07.2025" "15.07.2025" "30.06.2025" "10.08.2025" "20.09.2025")

# Funktion für zufälliges Element aus Array
rand_elem() {
  local arr=("$@")
  echo "${arr[RANDOM % ${#arr[@]}]}"
}

for ((i=1; i<=NUM_TODOS; i++)); do
  task=$(rand_elem "${TASKS[@]}")
  cmd="utodo add \"$task\""

  # Zufällig Parameter hinzufügen
  if (( RANDOM % 2 )); then
    prio=$(rand_elem "${PRIORITIES[@]}")
    cmd+=" -p $prio"
  fi
  if (( RANDOM % 2 )); then
    date=$(rand_elem "${DATES[@]}")
    cmd+=" -d $date"
  fi
  if (( RANDOM % 2 )); then
    memo=$(rand_elem "${DESCRIPTIONS[@]}")
    cmd+=" -m \"$memo\""
  fi

  echo ">> $cmd"
  eval "$cmd"
done

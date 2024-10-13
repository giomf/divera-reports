# Command-Line Help for `divera-reports`

This document contains the help content for the `divera-reports` command-line program.

**Command Overview:**

* [`divera-reports`↴](#divera-reports)
* [`divera-reports init`↴](#divera-reports-init)
* [`divera-reports report-types`↴](#divera-reports-report-types)
* [`divera-reports report`↴](#divera-reports-report)
* [`divera-reports report absences`↴](#divera-reports-report-absences)
* [`divera-reports report roster`↴](#divera-reports-report-roster)
* [`divera-reports report station`↴](#divera-reports-report-station)

## `divera-reports`

Divera reports

**Usage:** `divera-reports <COMMAND>`

###### **Subcommands:**

* `init` — Initialize the config
* `report-types` — Prints available report types
* `report` — Prints or writes reports



## `divera-reports init`

Initialize the config

**Usage:** `divera-reports init --divera-username <DIVERA_USERNAME> --divera-password <DIVERA_PASSWORD>`

###### **Options:**

* `--divera-username <DIVERA_USERNAME>` — Username for divera247
* `--divera-password <DIVERA_PASSWORD>` — Password for divera247



## `divera-reports report-types`

Prints available report types

**Usage:** `divera-reports report-types`



## `divera-reports report`

Prints or writes reports

**Usage:** `divera-reports report <COMMAND>`

###### **Subcommands:**

* `absences` — Absences reports
* `roster` — Roster reports
* `station` — Station reports



## `divera-reports report absences`

Absences reports

**Usage:** `divera-reports report absences <--print|--write <WRITE>>`

###### **Options:**

* `--print` — Prints the reports in a table format
* `--write <WRITE>` — Writes the reports to an xlsx file



## `divera-reports report roster`

Roster reports

**Usage:** `divera-reports report roster <--print|--write <WRITE>>`

###### **Options:**

* `--print` — Prints the reports in a table format
* `--write <WRITE>` — Writes the reports to an xlsx file



## `divera-reports report station`

Station reports

**Usage:** `divera-reports report station <--print|--write <WRITE>>`

###### **Options:**

* `--print` — Prints the reports in a table format
* `--write <WRITE>` — Writes the reports to an xlsx file



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

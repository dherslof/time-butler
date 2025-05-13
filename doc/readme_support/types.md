# time-butler Types
Following types are containers used to store time and information

## Project
Represents a project with following fields:
* **name** - Name of the project
* **description** - Description of the project.

Time [entries](#entry) are stored in projects.

### Examples

Create a new project
```bash
$ time-butler add project --name foobar --description "my new project"
```

Add new entry to project
```bash
$ time-butler add entry --project "foobar" --hours 5 --description "did some work"
```

list entries in project
```bash
$ time-butler list project --foobar
+---------+---------------+-------+--------------------------------------+--------------------------------------+
| Project | Description   | Hours | Created                              | ID                                   |
+===============================================================================================================+
| foobar  | did some work | 5     | 2024-12-20 09:36:21.385067343 +01:00 | f8f64508-7541-4ae9-8891-39d753e76200 |
+---------+---------------+-------+--------------------------------------+--------------------------------------+

```

Create a report from project
```bash
$ time-butler report project --name foobar --format csv
```

## Entry
A structure for time reporting to a project.
Contains following data:
* **Hours** - hours to report.
* **Description** - What has been done during these hours.
* **Created** - When the entry was created. This can not be set by the user.
* **ID** - Unique ID for the entry. This can not be set by the user

## Week
A structure which contains the reported [Days](#Day). The week will be created automatically based on when the day is created by the user.

### Examples

List a week
```bash
$ time-butler list --week 51
+------+------------+--------------------------------------+--------------------------------------+-------+--------+------------------------+
| Week | Date       | Start time                           | End time                             | Hours | Closed | Extra info             |
+===========================================================================================================================================+
| 51   | 2024-12-19 | 2024-12-19 10:30:21.403602448 +01:00 | 2024-12-19 10:44:33.994289190 +01:00 | 0     | true   | did something |
+------+------------+--------------------------------------+--------------------------------------+-------+--------+------------------------+

```
Create a week report in json format
```bash
$ time-butler report week --number 51 --format json
```

## Day
A day is like a [entry](#entry), but represent a working day instead and are not reported to a project.
A day contains the following fields which can be set by the user:
* **starting-time** - Timestamp when the work started. will be set when the day is created
* **ending-time** - Timestamp when work ended. Will be set when the day is created
* **extra-info** - A description or extra information regarding the day

If a day with same date already exists when creating a new one, the fields above will be evaluated. If not set earlier, the days will be merged. If all fields already set, the new day will be ignored. This means that you first create a day in the beginning of the day with a starting time, in order to later create same day again with ending time and description. The days will be merge by the time-butler and there will only be 1 full day added for the date.

Additional information set automatically upon creation of the Day:
* **Date** - Date of the Day
* **Week** - Current week
* **Hours** - Hours worked (ending time - starting time)
* **Closed/Open** - The day is considered closed if both start and end time are set.

### Examples

Create a new day with start-time
```bash
$ time-butler add day --starting-time
```

Create a new day with end-time and description of the day.
```bash
$ time-butler add day --ending-time --extra-info 'normal work day'
```

List a week
```bash
$ time-butler list --week 51
+------+------------+--------------------------------------+--------------------------------------+-------+--------+------------------------+
| Week | Date       | Start time                           | End time                             | Hours | Closed | Extra info             |
+===========================================================================================================================================+
| 51   | 2024-12-19 | 2024-12-19 10:30:21.403602448 +01:00 | 2024-12-19 10:44:33.994289190 +01:00 | 0     | true   | work work work |
|------+------------+--------------------------------------+--------------------------------------+-------+--------+------------------------|
| 51   | 2024-12-20 | 2024-12-20 10:22:17.352334224 +01:00 | 2024-12-20 10:23:38.514035907 +01:00 | 0     | true   | normal work day        |
+------+------------+--------------------------------------+--------------------------------------+-------+--------+------------------------+

```

Generate a week report:
```bash
$ time-butler report week --number 51 --format html
```
# Architecture

## Backend

### Model

Entry Example:

- Payee: Date Night
- Start: 6pm Saturday October 11 2025
- End: 10pm Saturday October 11 2025
- Duration: 4 hours
- Category: 🌹 Romance
- Category Group: 🏡 Personal
- Memo: LP + Big Grove

Monthly Budget Example:

- 

#### Entities

- Entry
- Category
- Category Group
- Category Budget

##### Entry

| data key      | notes                                                                                                         |
| ------------- | ------------------------------------------------------------------------------------------------------------- |
| `id`          | bigint                                                                                                        |
| `global_id`   | UUIDv4                                                                                                        |
| `payee`       | user defined name of the reciever of the time (e.g., Work, Chores, Meals)                                     |
| `start_time`  | UTC time representing the start of the entry                                                                  |
| `end_time`    | UTC time representing the end of the task--can be optional if the user is timing a task currently in progress |
| `duration`    | auto-computed                                                                                                 |
| `memo`        | user defined note representing additional notes that pertain to the enry (e.g. work commute)                  |
| `category_id` | the id of the category selected for the entry                                                                 |
| `created_at`  | auto-computed when a new row is added to the database                                                         |
| `updated_at`  | auto-computed when an existing row is updated                                                                 |
| `deleted_at`  | auto-computed when an existing row is deleted                                                                 |
| `version`     | auto-computed intrements when row is updated                                                                  |

##### Catogory

| data key     | notes                                                                                        |
| ------------ | -------------------------------------------------------------------------------------------- |
| `id`         | bigint                                                                                       |
| `global_id`  | UUIDv4                                                                                       |
| `name`       | user defined name of the reciever of the time (e.g., Work, Chores, Meals)                    |
| `note`       | user defined note representing additional notes that pertain to the enry (e.g. work commute) |
| `group_id`   | the id of the category group selected for the entry                                          |
| `created_at` | auto-computed when a new row is added to the database                                        |
| `updated_at` | auto-computed when an existing row is updated                                                |
| `deleted_at` | auto-computed when an existing row is deleted                                                |
| `version`    | auto-computed intrements when row is updated                                                 |

##### Category Group

| data key     | notes                                                                                                                  |
| ------------ | ---------------------------------------------------------------------------------------------------------------------- |
| `id`         | bigint                                                                                                                 |
| `global_id`  | UUIDv4                                                                                                                 |
| `name`       | user defined name of the reciever of the time (e.g., Work, Personal)                                                   |
| `note`       | user defined note representing additional notes that pertain to the enry (e.g. this group is for all things education) |
| `created_at` | auto-computed when a new row is added to the database                                                                  |
| `updated_at` | auto-computed when an existing row is updated                                                                          |
| `deleted_at` | auto-computed when an existing row is deleted                                                                          |
| `version`    | auto-computed intrements when row is updated                                                                           |

##### Category Budget

| data key      | notes                                                 |
| ------------- | ----------------------------------------------------- |
| `id`          | bigint                                                |
| `year_month`  | year and month identifier                             |
| `category_id` | the id of the category selected for the entry         |
| `created_at`  | auto-computed when a new row is added to the database |
| `updated_at`  | auto-computed when an existing row is updated         |
| `deleted_at`  | auto-computed when an existing row is deleted         |
| `version`     | auto-computed intrements when row is updated          |

## Frontend

### Components

#### Category Row

- Play/Pause
- Time spent
- Time assigned
- Name

#### Category Dialog

- Spark line of history with trend line overlay
- Time spent
- Time assigned
- Memo
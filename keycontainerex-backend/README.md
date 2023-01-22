# keycontainerex-backend

## Subcommand

All available subcommand:

* add
  - Add password.
* show
  - show single password info.
  - show all password info, **need auth**.
  - show password info and value, **need auth**.
* remove, rm
  - remove password by account, **need auth**.
* edit
  - edit password by account, **need auth**.
* status
  - show database status.
* config
  - get or set config, **need auth**.


### add

Add password.

~~~ bash
# Add password:
KeyContainerEx add
> Input account:
test
> Input password:
xxxx
> Confirm password:
xxxx
> Input comment (or press enter to skip):
comment for account test
> success

# Add password with account:
KeyContainerEx add -a test
> Input password:
xxxx
> Confirm password:
xxxx
> Input comment (or press enter to skip):
comment for account test
> success

# Add password with account and comment:
KeyContainerEx add -a test -c "comment for account test"
> Input password:
xxxx
> Confirm password:
xxxx
> success

# Add password from file:
KeyContainerEx add path/to/file
~~~

### show

~~~ bash
# Show info with given keyword
KeyContainerEx show test
> Account: test
> Comment: comment for account test
> Add Time: 2023-01-01 00:00:00 UTF+8
> Last Modified Time: 2023-01-02  00:01:00 UTF+8

# Show info with given glob
KeyContainerEx show "t*"
> Account: test
> Comment: comment for account test
> Add Time: 2023-01-01 00:00:00 UTF+8
> Last Modified Time: 2023-01-02  00:01:00 UTF+8

# Show info with given regular expression
KeyContainerEx show -E "t.+"
...

# Show info, search in account
KeyContainerEx show -a comment
> No match found

# Show info, search in comment
KeyContainerEx show -c comment
> Account: test
> Comment: comment for account test
> Add Time: 2023-01-01 00:00:00 UTF+8
> Last Modified Time: 2023-01-02  00:01:00 UTF+8
~~~

### remove

~~~ bash
# Remove by account
KeyContainer remove test
# Or use rm
KeyContainer rm test
~~~

### edit

~~~ bash
# Edit password by account
KeyContarinerEx edit test
> Input password:
yyyy
> Confirm password:
yyyy
> Success

# Edit comment by account
KeyContainerEx edit test -c
> Input comment:
new comment
> Success
~~~

### status

~~~ bash
# Show database status
KeyContainerEx status
> KeyContainerEx v0.1.0
> Password count: 10
> Save path: path/to/database
~~~

### config

~~~ bash
# Get config value for key xxxx.yyyy
KeyContainerEx config xxxx.yyyy

# Set config value for key xxxx.yyyy
KeyContainerEx config xxxx.yyyy v
> success
~~~

All available config:
* main
  - password: main config. *string*
  - path: database file path. *string*

## Storage

### Structure

> In Go, ``byte``==``uint8``

* Header: ``kcex`` **4 byte**
* Version: *byte* **1 byte**
* Main Password:
  - Hash Type: *int8* **1 byte**
  - Hash Length: *uint32* **4 byte**
  - Hash Data. *[]byte*
* Main Password Split: ``0xa1`` *byte* **1 byte**
* Password 1
  - Crypto Type: *int8* **1 byte**
  - Crypto Mode: *int8* **1 byte**
  - Account:
    * Account Length: *uint32* **4 byte**
    * Account Data. *[] byte*
  - Password:
    * Password Length: *uint32* **4 byte**
    * Password Data. *[] byte*
  - Comment:
    * Comment Length: *uint32* **4 byte**
    * Comment Data. *[] byte*
  - Created Time:
    * Time Length: *uint32*
    * Time Data. *[] byte*
  - Last Modified TIme:
    * Time Length: *uint32* **4 byte**
    * Time Data. *[] byte*
  - Password Split: ``0xa2`` *byte* **4 byte**
* Password 2
  - Crypto Type: *uint32* **4 byte**
  - Crypto Mode: *int8* **1 byte**
  - Account:
    * Account Length: *uint32* **4 byte**
    * Account Data. *[] byte*
  - Password:
    * Password Length: *uint32* **4 byte**
    * Password Data. *[] byte*
  - Comment:
    * Comment Length: *uint32* **4 byte**
    * Comment Data. *[] byte*
  - Created Time:
    * Time Length: *uint32* **4 byte**
    * Time Data. *[] byte*
  - Last Modified TIme:
    * Time Length: *uint32* **4 byte**
    * Time Data. *[] byte*
  - Password Split: ``0xa2`` *byte* **1 byte**
* EOF

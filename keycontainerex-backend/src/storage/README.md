## Storage

Translate [KeePass](https://keepass.info/download.html)'s ``kdbx`` code into rust.

Current using source code of [KeePass 2.35](https://sourceforge.net/projects/keepass/files/KeePass%202.x/2.35/KeePass-2.35-Source.zip/download) which introduced ``kdbx 4`` format.

See also code of [KeePassXC](https://github.com/keepassxreboot/keepassxc/)

### Develop

**kdbx4** => ``m_pwDatabase`` (**PwDatabase.cs** -> **database.rs**) => ``m_pgRootGroup`` (**PwGroup.cs** -> **group.rs**)
=> ``m_listGroups`` (**PwObjectList.cs** -> **object_list.rs**)

**kdbx4** => ``m_dCustomData`` (**StringDictionaryEx** -> **string_dictionary.rs**)

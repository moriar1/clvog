# Clvog
CLI utility that helps in maintaining personal small "portable" video database focused on AMV.
It uses [yt-dlp](https://github.com/yt-dlp/yt-dlp) to download videos from a pre-prepared
list with links to videos and allows you to perform some trivial actions both with downloaded
files and with the list of references. 

# Requirements
- git
- python3
- [rust](https://www.rust-lang.org/learn/get-started)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp)

# Usage

## Example
This example is shown for a unix-like system.

### Run
To download project use
```bash
git clone https://github.com/moriar1/clvog
cd clvog
mkdir vid # required create directory `vid` 
```

### Explaining `new_vid_list.txt` content
It contains required names and link to video: `<type>-<date_of_source>-<date_of_discvery>.mp4 <link> (<description>)`
(See its content using your text editor for exact names.
Types are: AV, AMV, MAD. See [reference](#list-structure) for more.)


### Creating new video database
Move example of list with new video you would like to add to your video database (or create at this point).
```bash
mv example/new_vid_list.txt ./
```

Then create your database:
```bash
cargo run -- add
# or `clvog add` if you use pre-build binary
```

It creates `vid_list.txt` with content of `new_vid_list.txt`, but also adds numbers to every record.
(also creates `.bak` backup of lists).
Then clvog downloads these new videos giving them names according to `new_vid_list.txt`.
If video could not be downloaded its record writes in `failed_downloads.log`,
creates empty file like `0001-AAA.mp4` (type `AAA` and no dates, but with its number)

In this example you see new files:
- `0001-AMV-20230908-20240303.mp4`
- `0002-AAA.mp4`
- `0003-AMV-20240225-20240303.mp4`

`0002-AAA.mp4` created because it must be downloaded manually.

Move new videos in `vid/` directory (`clvog` will `check` video file names in it and in `vid_list.txt` next time):
```bash
mv 0* vid/
```

Before using `add` again clear `new_vid_list.txt` content and write your new videos in it.

## Reference

### Commands
See [TODO](#todo).
- `-u, --skip-check`
- `-v, --verbose`  
- `-V, --version`  
- `insert <num> <filename>`
- `add`
- `add -p <PATH>`
- `move <num> <num>`
- `pull # rename video files from list to dir (-u option disabled)`
- `sync # rename records from dir to list (-u option disabled)`
- `rm <num>`
- `rename <name> <name>`
- `hide <type> [,<type>,..] # files only`
- `check # verfiy directory and list names matching (runs before any action)`  

Example: `clvog add -vup push_list.txt`
 – add new videos from `push_list.txt` to `vid_list.txt` with output debug infromation and without `check` names mathcing

### Main actions:
#### `add` and `add -p <PATH>`

Download and insert videos from `new_vid_list.txt` or from `<PATH>` text file at the end of `vid_list.txt`
##### Example
`new_video_list.txt`:
> AMV-31122020-31122020.mp4 https://some_link.com  
> AMV-31122020-31122020.mp4 https://another_link.com  
> AMV-01012000-31012002.mp4 https://*some_broken_link.com  

`add` inserts new 3 records in the end of `vid_list.txt` and gives the apropriate numbers
> ...  
> 22-`<old record>`.mp4  
> 23-AMV-31122020-31122020.mp4 https://some_link.com  
> 24-AV-31122020-31122020.mp4 https://another_link.com  
> 25-AMV-01012000-31012002.mp4 https:// some_broken_link.com  
> `<end of list>`  

It also downloads these videos and rename files according the list
`/path/to/video_directory/` (`./` by default):  
> ...  
> ./23-AMV-31122020-31122020.mp4  
> etc.  

##### What if `add` used to init new video db?
Clvog will
1. read `new_video_list.txt`  
2. create and write `vid_list.txt`  
3. download and rename files


#### `rm <num>`  
delete file under `<num>`
##### Example

`vid_list.txt` and `path/to/video_dir` (`./vid` by default):
> 34-AV-11-11.mp4 https://0  
> 35-AMV-22-22.mp4 https://1  
> 36-AMV-22-22.mp4 https://2  

Using `rm 35`  
(rename `35-AMV-22-22.mp4` to `AMV-22-22.mp4` then i-=1 for [35..])

> 34-AV-11-11.mp4 https://0  
> 35-AMV-22-22.mp4 https://2  

Be sure there isn't one more file with name `AMV-22-22.mp4` otherwise it will be rewritten

#### `insert`  
insert new file `<name>` into `<num>`
##### Example

`vid_list.txt` and `path/to/video_dir`:  
> 34-AV-11-11.mp4 https://  
> 35-AMV-22-22.mp4 https://  
> 36-AMV-22-22.mp4 https://  

new file:  
> MAD-33-33.mp4  

Using `insert AMV-22-22.mp4 35`  
(i+=1 for [35..] then insert new file under 35)

`vid_list.txt` and `path/to/video_dir`:
> 34-AV-11-11.mp4 https://  
> 35-MAD-33-33.mp4 https://  
> 36-AMV-22-22.mp4 https://  
> 37-AMV-22-22.mp4 https://  

#### `sync`
Rename any files in the directory then use `sync` to update vid_list.txt,
but do not rewrite nums
> Example of renaming:  
> 34-AV-11-11.mp4 -> 34-AV-some-new-data-11-22.mp4

### Secondary actions
`hide AMV AV` - add dot at the beginning of the file name if it is like 0000-AMV... or 0000-AV...
On Unix-like systems file manager will not show them in directory.

`rename <name1> <name2>` - the same as `sync` but manual  
`move <num1> <num2>` - the same as `delete <num1>` then `insert <name1> <num2>`  
`pull` - rename files in list then use `pull` to rename directory entries accordingly (opposite `sync`)  

## How dir-list verification (`check`) works
It takes every line in `vid_list.txt` then checks if the every file name is the same.
It recognizes files starts with four digits and at least 11 character length, other characters are any: `0001aCV.ext`.
If there are at least one error - execution stops and prints unmatched entry-record.
Use `AAA` at `&[5..8]` in filename to skip verification. [Note](#file-entry)

## List Structure
There are 3 video types: **AV** (Anime [fan-made] Video), **AMV** (Anime Music Video) and **MAD** (means otoMAD - 音MAD)
### Record
`<num>-<type>-<any_info>.<ext> <link> (<description>)`
> Example: 0001-AV.mp4 https://link_or_broken_link (some information, more links)

### File entry
- the same as first field in list: `<num>-<type>-<any_info>.<ext>`  
> Example: 0001-AV.mp4

> [!NOTE]  
> If file is broken or you would like to mark it, but do not want to rename record  
> Use `AAA` type in directory entry name - error checker will pass it.
>  
> For example, your video file (directory entry): `0011-AAA-1-2.mp4`, but record: `0011-AMV-31122020-31122020.mp4` - ok. 

### vid_list.txt
Example:
> 0001-AMV.mp4 https://add_link (dunno author)  
> 0002-AV-0-bro_tf_you_watching-20210720-20240225.mp4 https://vk.com/video-167127847_456272439  
> 0003-AMV-20230908-20240303.mp4 https://amvnews.ru/index.php?go=Files&file=down&id=12462 (https://amvnews.ru/index.php?go=Files&in=view&id=12462)  
> 0004-AMV-20230827-20240621.mp4 https://*youtube.com/watch?v=cT4QNOz_xCU (see_, That Tuturu! Hurts | Steins;Gate | NUMB |「 Edit/AMV 」)  

Comments (for example, in `comments.txt` file or in description to last video)
> From the available the link to the source from which it is least problematic to download (using yt-dlp) is selected,
> while the source date (the first in the file name) selects the earliest of all available sources.
> 
> All single youtube links are have `*`: `https:// * youtube.com/...` in order for `yt-dlp` not to try to download them.
> May use `sed` to delete `*`:
> ```bash
> sed 's/https:\/\/\*\(.*\)/https:\/\/\1/g' input_file.txt > output_file.txt
> ```


# TODO:
- [ ] implement all commands
    - [x] check
    - [ ] add (with `-p` option)
    - [ ] insert
    - [ ] sync
    - [ ] pull
    - [ ] move
    - [ ] rm
    - [ ] hide
    - [ ] rename
- [ ] add caching to `get_entries()` and `get_records()` in `actions.rs` (see [example](./misc/cache.rs))
- [ ] add `version` command with the same output as `--version` or `-V` option
- [ ] download videos in `./vid/` directory
 
## Future of Anime Music Video Organizer
Clvog is the first step to creating fully-featured Anime Music Video Organizer with TUI
(looks like [ranger](https://github.com/ranger/ranger) and shows more video previews)
and/or GUI if it will make sense. I hope it will not be so messy.

Small description about it: [click](misc/about_tui.md)

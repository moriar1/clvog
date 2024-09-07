import yt_dlp
import re
import subprocess
import os
import sys


def process_coub_video(input_file, output_file):
    command = [
        'ffmpeg',
        '-stream_loop', '-1',
        '-i', input_file,
        '-i', input_file,
        '-shortest',
        '-map', '0:v:0',
        '-map', '1:a:0',
        '-c', 'copy',
        output_file
    ]
    subprocess.run(command)


# Disable ouput in stderr by yt-dlp
# Error output is in exception handler below
class loggerOutputs:
    def error(msg):
        pass

    def warning(msg):
        print(msg)

    def debug(msg):
        print(msg)


with open(sys.argv[1], 'r') as file:
    log = open('failed_downloads.log', 'w')
    for line in file:
        # extract file name and link
        match = re.match(r'(\S+)\s+(https?://\S+)', line.strip())
        if match:
            video_name, video_url = match.groups()
            ydl_opts = {
                'outtmpl': video_name,
                # "quiet": True,
                "logger": loggerOutputs,
            }
            try:
                with yt_dlp.YoutubeDL(ydl_opts) as ydl:
                    ydl.download([video_url])

                print(f"Downloaded: {video_name}")

                # loop coub video
                if 'coub.com' in video_url:
                    input_file = video_name
                    output_file = f"processed_{video_name}"

                    process_coub_video(input_file, output_file)
                    os.remove(input_file)
                    os.rename(output_file, video_name)
                    print(f"Processed Coub video: {video_name}")

            except Exception as e:
                print(f"Failed to download {video_name}: {e}", file=sys.stderr)
                log.write(f"{video_name} {video_url}\n")
                dummy = video_name[0:5] + "AAA.mp4"
                open(dummy, 'a').close()
log.close()
if (os.stat('failed_downloads.log').st_size == 0):
    os.remove('failed_downloads.log')

# Using external dowloader: yt_dlp --downloader aria2c "https://"
# coub loop issue: github.com/yt-dlp/yt-dlp/issues/1930

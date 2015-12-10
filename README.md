# splitYV12pipe
split yuv420 stream

ffmpeg -i input.file -pix_fmt yuv420p -f rawvideo - | splityv12pipe -w width -h height -frame <frame number> -o outfile_*

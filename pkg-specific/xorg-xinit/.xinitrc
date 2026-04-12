#!/programs/bash

xeyes -geometry 150x100+0+0 &
xeyes -geometry 150x100+160+0 &
xeyes -geometry 150x100+320+0 &

exec twm

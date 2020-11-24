// See https://github.com/legokichi/ts-ebml for making videos seekable:
// $  npm install --global ts-ebml
// $ ts-ebml -s advent-of-code-2019-13-part2.webm > s.webm
//
// Also:
// $ ffmpeg -i in.webm -c copy out.webm
//
// Then https://www.matroska.org/downloads/mkclean.html:
// $ mkclean --optimize
//
// Or use the webm-cleaner scirpt:
// $ webm-cleaner in.webm
export default function CanvasRecorder(canvas, videoBitsPerSecond) {
    this.start = () => {
        const mimeType = [
            'video/webm;codecs=vp9',
            "video/webm",
            'video/vp8',
            "video/webm;codecs=vp8",
            "video/webm;codecs=daala",
            "video/webm;codecs=h264",
            "video/mpeg"
        ].find(MediaRecorder.isTypeSupported);

        if (!mimeType) {
            throw new Error("No supported mime type found for MediaRecorder");
        }

        const stream = canvas.captureStream();
        this.mediaRecorder = new MediaRecorder(stream, {
            mimeType,
            videoBitsPerSecond: videoBitsPerSecond || 5000000
        });

        this.mediaRecorder.ondataavailable = (event) => {
            const url = window.URL.createObjectURL(event.data);
            const a = document.createElement('a');
            a.href = url;
            a.download = this.fileName;
            a.click();
            window.URL.revokeObjectURL(url);
        }

        this.mediaRecorder.start();
    }

    this.stopAndSave = (fileName) => {
        this.fileName = fileName || 'recording.webm';
        this.mediaRecorder.stop();
    }
}

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "avif/avif.h"

#ifdef _WIN32
#define EXPORT __declspec(dllexport)
#else
#define EXPORT
#endif

typedef struct {
    uint8_t *data;
    int width;
    int height;
    int duration_ms;
} AvifFrame;

typedef struct {
    AvifFrame *frames;
    int frame_count;
} AvifResult;

EXPORT AvifResult* decode_avif(const uint8_t *avif_data, size_t avif_size) {
    avifDecoder *decoder = avifDecoderCreate();
    if (!decoder) return NULL;

    avifDecoderSetIOMemory(decoder, avif_data, avif_size);
    if (avifDecoderParse(decoder) != AVIF_RESULT_OK) {
        avifDecoderDestroy(decoder);
        return NULL;
    }

    int frame_count = decoder->imageCount;
    AvifFrame *frames = (AvifFrame*)malloc(frame_count * sizeof(AvifFrame));

    int i = 0;
    while (avifDecoderNextImage(decoder) == AVIF_RESULT_OK) {
        avifImage *image = decoder->image;
        avifRGBImage rgb;
        avifRGBImageSetDefaults(&rgb, image);
        avifRGBImageAllocatePixels(&rgb);
        avifImageYUVToRGB(image, &rgb);

        frames[i].width = rgb.width;
        frames[i].height = rgb.height;
        frames[i].duration_ms = (int)(decoder->imageTiming.duration * 1000);
        frames[i].data = (uint8_t*)malloc(rgb.width * rgb.height * 4);
        memcpy(frames[i].data, rgb.pixels, rgb.width * rgb.height * 4);
        
        avifRGBImageFreePixels(&rgb);
        i++;
    }

    avifDecoderDestroy(decoder);

    AvifResult *result = (AvifResult*)malloc(sizeof(AvifResult));
    result->frames = frames;
    result->frame_count = frame_count;
    return result;
}

EXPORT void free_avif_result(AvifResult *result) {
    for (int i = 0; i < result->frame_count; i++) {
        free(result->frames[i].data);
    }
    free(result->frames);
    free(result);
}

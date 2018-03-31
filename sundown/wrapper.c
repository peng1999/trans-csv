#include <string.h>
#include "sundown/markdown.h"
#include "sundown/html.h"
#include "sundown/buffer.h"

struct buf* render(const unsigned char * input)
{
    int size = strlen(input);
    struct sd_callbacks callbacks;
    struct html_renderopt options;
    sdhtml_renderer(&callbacks, &options, 0);

    struct buf *ob;
    ob = bufnew(size * 2);
    struct sd_markdown* md = sd_markdown_new(MKDEXT_SUPERSCRIPT, 16, &callbacks, &options);

    sd_markdown_render(ob, input, size, md);
    sd_markdown_free(md);

    return ob;

    // bufrelease(ob);
}

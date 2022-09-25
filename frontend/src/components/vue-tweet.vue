<!-- Adapted from https://github.com/DannyFeliz/vue-tweet -->

<!-- MIT License

Copyright (c) 2021 Danny Feliz

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. -->

<template>
  <slot v-if="isLoading" name="loading"></slot>
  <slot v-else-if="hasError" name="error"></slot>
  <div ref="tweetContainer" v-bind="attrs"></div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, nextTick, watch } from "vue";
import type { PropType } from 'vue'
declare global {
  interface Window { twttr: any; }
}
export default defineComponent({

  props: {
    tweetId: {
      type: String,
      required: true
    },
    conversation: {
      type: String as PropType<"all" | "none">,
      default: "all"
    },
    cards: {
      type: String as PropType<"visible" | "hidden">,
      default: "visible"
    },
    width: {
      type: [String, Number] as PropType<"auto" | number>,
      default: "auto"
    },
    align: {
      type: [String, undefined] as PropType<"left" | "right" | "center" | undefined>,
      default: undefined
    },
    theme: {
      type: String as PropType<"light" | "dark">,
      default: "light"
    },
    lang: {
      type: String as PropType<"ar" | "bn" | "cs" | "da" | "de" | "el" | "en" | "es" | "fa" | "fi" | "fil" | "fr" | "he" | "hi" | "hu" | "id" | "it" | "ja" | "ko" | "msa" | "nl" | "no" | "pl" | "pt" | "ro" | "ru" | "sv" | "th" | "tr" | "uk" | "ur" | "vi" | "zh-cn" | "zh-tw">,
      default: "en"
    },
    dnt: {
      type: Boolean,
      default: false
    }
  },
  emits: ["tweet-load-success", "tweet-load-error"],
  setup(props, { attrs, emit }) {
    const isLoading = ref<boolean>(true);
    const hasError = ref<boolean>(false);
    const tweetContainer = ref<HTMLDivElement>();

    onMounted(() => {
      renderTweet();
    })

    watch(props, () => {
      renderTweet();
    })

    function renderTweet() {
      // if (!(window["twttr"] && window["twttr"].ready)) {
      if (!(window.twttr && window.twttr)) {
        addScript("https://platform.twitter.com/widgets.js", renderTweet);
        return;
      }

      // window["twttr"].ready().then(({ widgets }: any) => {
      window.twttr.ready().then(({ widgets }: any) => {
        isLoading.value = true;
        hasError.value = false;
        // Clear previously rendered tweet before rendering the updated tweet id
        if (tweetContainer.value) {
          tweetContainer.value.innerHTML = "";
        }

        const { tweetId, ...options } = props;
        widgets
          .createTweet(tweetId, tweetContainer.value, options)
          .then(async (twitterWidgetElement: HTMLElement | undefined) => {
            // Since we're mutating the DOM directly with the embed we need to tell Vue wait until the DOM update
            await nextTick();

            if (twitterWidgetElement) {
              emit("tweet-load-success", twitterWidgetElement);
            } else {
              hasError.value = true;
              emit("tweet-load-error");
            }

            isLoading.value = false;
          })
      })
    }

    function addScript(src: string, cb: () => void) {
      const s = document.createElement("script");
      s.setAttribute("src", src);
      s.addEventListener("load", () => cb(), false);
      document.body.appendChild(s);
    }

    return { tweetContainer, isLoading, hasError, attrs }
  }
})
</script>

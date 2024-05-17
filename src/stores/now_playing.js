import { defineStore } from "pinia";

export const useNowPlayingStore = defineStore('now_playing', {
    state: () => {
        return {
            title: '',
            artist: '',
            duration: 0,
            file_path: '',
            picture_base64: '',
            album: '',
        }
    },
})
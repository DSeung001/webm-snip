<script lang="ts">
  export let isActive = false;
  export let isLoading = false;
  export let onOpen: () => void;
</script>

<section class:is-active={isActive} class="drop-zone">
  <div>
    <h2>{isLoading ? 'WebM 파일을 불러오는 중입니다.' : 'WebM 파일을 여기에 끌어다 놓으세요.'}</h2>
    <p>로컬 파일만 처리하며 원본 파일은 덮어쓰지 않습니다.</p>
  </div>
  <button class="primary" disabled={isLoading} on:click={onOpen}>WebM 파일 열기</button>
</section>

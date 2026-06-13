// 内嵌交互 demo 的懒加载：扫描 <figure class="bevy-demo" data-src="…">，
// 把占位图包成一个“运行”按钮；点击后才注入 iframe 去加载几 MB 的 wasm。
// 不点、无 JS、打印时，留在页面上的就是占位图 + 图注，正文信息自足。
(function () {
  function mount(fig) {
    var src = fig.getAttribute("data-src");
    var img = fig.querySelector("img");
    if (!src || !img) return;
    var ratio = fig.getAttribute("data-ratio") || "16 / 10";

    var btn = document.createElement("button");
    btn.type = "button";
    btn.className = "bevy-demo-run";
    btn.setAttribute("aria-label", "在浏览器里运行演示");

    var play = document.createElement("span");
    play.className = "bevy-demo-play";
    play.textContent = "▶ 在浏览器里运行";

    img.parentNode.insertBefore(btn, img);
    btn.appendChild(img);
    btn.appendChild(play);

    btn.addEventListener("click", function () {
      var frame = document.createElement("iframe");
      frame.src = src;
      frame.title = "交互演示";
      frame.loading = "lazy";
      frame.allow = "autoplay";
      frame.className = "bevy-demo-frame";
      frame.style.aspectRatio = ratio;
      btn.replaceWith(frame);
    });
  }

  document.addEventListener("DOMContentLoaded", function () {
    document.querySelectorAll(".bevy-demo[data-src]").forEach(mount);
  });
})();

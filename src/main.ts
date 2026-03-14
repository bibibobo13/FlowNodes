import { getCurrentWindow } from "@tauri-apps/api/window";
declare const feather: any;
const appWindow = getCurrentWindow();


const buttons = document.querySelectorAll(".nav-btn") as NodeListOf<HTMLElement>;
const indicator = document.getElementById("side-indicator") as HTMLElement | null;
const contentArea = document.getElementById("content-area") as HTMLElement | null;
const popup = document.getElementById('popup') as HTMLElement;
const input = document.querySelector('.NameOfTask') as HTMLInputElement;
const submitBtn = document.querySelector('.Submit') as HTMLButtonElement;


function moveIndicator(btn: HTMLElement): void {
  if (!indicator) return;

  const btnTop = btn.offsetTop;
  const btnHeight = btn.offsetHeight;
  const indHeight = indicator.offsetHeight;
  
  const targetPos = btnTop + (btnHeight / 2) - (indHeight / 2);
  indicator.style.transform = `translateY(${targetPos}px)`;
}


buttons.forEach((btn) => {
  btn.addEventListener("click", () => {

    buttons.forEach((b) => b.classList.remove("active"));
    btn.classList.add("active");
    
    moveIndicator(btn);
    
    const key = btn.getAttribute("data-key") || "unknown";
    
    
    if (contentArea) {
      contentArea.classList.remove("content-animate");

      void contentArea.offsetWidth;
      contentArea.classList.add("content-animate");
      
      contentArea.innerHTML = `
        <h2 style="font-family: 'Cinzel', serif; font-size: 24px;">${key.toUpperCase()}</h2>
        <p style="opacity:0.4; margin-top:12px;">You Clicked On ${key}.</p>
      `;
    }
    if (key === 'add') {
      popup.classList.add('show');
      setTimeout(() => input.focus(), 300);
    }
  });
});


window.addEventListener("load", () => {
  
  if (typeof feather !== 'undefined') {
    feather.replace();
  }

  const activeBtn = document.querySelector(".nav-btn.active") as HTMLElement | null;
  if (activeBtn) {
    moveIndicator(activeBtn);
  }
});



const closeBtn = document.getElementById('close');
const minBtn = document.getElementById('minimize');
const maxBtn = document.getElementById('maximize');


closeBtn?.addEventListener('click', () => appWindow.close());
minBtn?.addEventListener('click', () => appWindow.minimize());
maxBtn?.addEventListener('click', () => appWindow.toggleMaximize());

submitBtn.addEventListener('click', () => {
    const taskName = input.value;
    input.value = "";
    popup.classList.remove('show');
});

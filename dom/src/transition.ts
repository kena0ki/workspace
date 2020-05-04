const PARSE_TRANSITION=/translate\(\s*(\d+\D+)\s*,\s*(\d+\D+)\s*\)/;
const TRANSITION_DURATION = 1000;
function reveal() {
  const blinds = document.querySelectorAll('.blind') as NodeListOf<HTMLDivElement>;
  blinds.forEach(function(div) {
    div.style.display = 'none';
  });
}
function shuffle() {
  let justBfore = Date.now();
  const blinds = document.querySelectorAll('.blind') as NodeListOf<HTMLDivElement>;
  blinds.forEach(function(div) {
    div.style.display = '';
  });
  swap();
  let i=0;
  let duration = TRANSITION_DURATION;
  f();
  const divs = document.querySelectorAll('.square') as NodeListOf<HTMLDivElement>;
  function f() {
    duration -= (10-(i/2))*10; // An = An-1 - 10 * (10 - (n-10)/2)  (A1 = 1000)
    // duration = Math.pow(2, -(i/2-10));
    i += 1;
    if (duration < 0) {
    // if (20 < i) {
      setTimeout(function() {
        divs.forEach(function(div) {
          div.style.transitionDuration = TRANSITION_DURATION+'ms';
        });
      }, TRANSITION_DURATION);
      return;
    }
    setTimeout(function() {
      swap();
      divs.forEach(function(div) {
        div.style.transitionDuration = duration+'ms';
      });
      const now = Date.now();
      console.log(i, now - justBfore);
      justBfore = now;
      f();
    }, duration);
  }
}
function swap() {
  const divs = document.querySelectorAll('.square') as NodeListOf<HTMLDivElement>;
  const picked1Idx = Math.floor(Math.random() * divs.length);
  const picked2Idx = (function() {
    const pickedIdx = Math.floor(Math.random() * (divs.length-1));
    return pickedIdx >= picked1Idx ? pickedIdx+1 : pickedIdx;
  })();
  const picked1Div = divs[picked1Idx];
  const picked2Div = divs[picked2Idx];
  const [,x1,y1] = picked1Div.style.transform.match(PARSE_TRANSITION) as string[];
  const [,x2,y2] = picked2Div.style.transform.match(PARSE_TRANSITION) as string[];
  picked1Div.style.transform = 'translate('+x2+','+y2+')';
  picked2Div.style.transform = 'translate('+x1+','+y1+')';
}
function move() {
  const divs = document.querySelectorAll('.square') as NodeListOf<HTMLDivElement>;
  divs.forEach(function(div) {
    if (div.classList.contains('moved')) {
      div.classList.remove('moved');
    } else {
      div.classList.add('moved');
      div.style.transitionTimingFunction = 'steps(4, jump-start)';
      div.style.backgroundColor = 'orange';
      div.style.color = 'blue';
    }
  });
}

const squareContainer = document.querySelector('.square-container') as HTMLDivElement;
const square = document.createElement('div') as HTMLDivElement;
square.setAttribute('class', 'square transitional');
const content = document.createElement('div') as HTMLDivElement;
content.setAttribute('class', 'content');
square.appendChild(content);
const blind = document.createElement('div') as HTMLDivElement;
blind.setAttribute('class', 'content blind');
blind.style.display = 'none';
blind.appendChild(document.createTextNode('???'));
square.appendChild(blind);
const SPAN = 110;
for (let i=0; i<5; i++) {
  const squareClone = square.cloneNode(true) as HTMLElement;
  squareClone.style.transform = 'translate('+SPAN*i+'px,0)';
  squareClone.setAttribute('id', 'square-' + (i+1));
  const content = squareClone.children[0];
  content.appendChild(document.createTextNode('square'+(i+1)));
  squareContainer.appendChild(squareClone);
}

(window as any).handlers = { move, swap, shuffle, reveal };


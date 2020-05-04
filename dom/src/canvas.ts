interface Vector {
  x, y: number
}


function main() {
  const canvas = document.getElementById('c') as HTMLCanvasElement;
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
  const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
  const balls = (function() {
    const ret: Rect[] = [];
    for (let i=0; i<100; i++){
      ret[i] = new Rect(canvas);
    }
    return ret;
  })();
  function animate() {
    ctx.fillStyle = 'rgba(255, 255, 255, 0.5)';
    ctx.fillRect(0,0,canvas.width,canvas.height);
    balls.forEach(function(b) {
      b.update();
      b.draw(ctx);
    });
    requestAnimationFrame(animate);
  }
  animate();
};

/**
 * Vector
 */
class Vector {
  constructor(x?, y?) {
    this.x = x || 0;
    this.y = y || 0;
  }
  set = function(x, y) {
      if (typeof x === 'object') {
          y = x.y;
          x = x.x;
      }
      this.x = x || 0;
      this.y = y || 0;
      return this;
  }
  add = function(v) {
      this.x += v.x;
      this.y += v.y;
      return this;
  }
  sub = function(v) {
      this.x -= v.x;
      this.y -= v.y;
      return this;
  }
  scale = function(s) {
      this.x *= s;
      this.y *= s;
      return this;
  }
  length = function() {
      return Math.sqrt(this.x * this.x + this.y * this.y);
  }
  lengthSq = function() {
      return this.x * this.x + this.y * this.y;
  }
  normalize = function() {
      var m = Math.sqrt(this.x * this.x + this.y * this.y);
      if (m) {
          this.x /= m;
          this.y /= m;
      }
      return this;
  }
  radian = function() {
      return Math.atan2(this.y, this.x);
  }
  radianTo = function(v) {
      var dx = v.x - this.x,
          dy = v.y - this.y;
      return Math.atan2(dy, dx);
  }
  distanceTo = function(v) {
      var dx = v.x - this.x,
          dy = v.y - this.y;
      return Math.sqrt(dx * dx + dy * dy);
  }
  distanceToSq = function(v) {
      var dx = v.x - this.x,
          dy = v.y - this.y;
      return dx * dx + dy * dy;
  }
  lerp = function(v, t) {
      this.x += (v.x - this.x) * t;
      this.y += (v.y - this.y) * t;
      return this;
  }
  clone = function() {
      return new Vector(this.x, this.y);
  }
  toString = function() {
      return '(x:' + this.x + ', y:' + this.y + ')';
  }
}

const GRAVITY = .05;
const GRAVITY_RADIAN = Math.PI/2;
const GRAVITY_VECTOR = unitVector(GRAVITY_RADIAN).scale(GRAVITY);
class Rect extends Vector {
  x: number;
  y: number;
  width: number;
  height: number;
  velocity: Vector = new Vector;
  xLimit: number;
  yLimit: number;
  friction: number;
  constructor(canvas: HTMLCanvasElement) {
    super();
    this.width = randomNum(10, 50);
    this.height = randomNum(10, 50);
    this.x = randomNum(0, canvas.width) - this.width/2;
    this.y = randomNum(0, canvas.height) - this.height/2;
    this.xLimit = canvas.width;
    this.yLimit = canvas.height;
    this.friction = randomNum(-0.7,-0.5);
  }
  update() {
    if (this.y + this.height < this.yLimit || this.velocity.radian() < 0) {
      this.velocity.add(GRAVITY_VECTOR);
      this.move(this.velocity);
    } else {
      this.velocity.scale(this.friction);
      this.move(this.velocity);
    }
  }
  draw(ctx: CanvasRenderingContext2D) {
    ctx.fillStyle = 'black';
    ctx.fillRect(this.x,this.y,this.width,this.height);
  }
  move(v: Vector) {
    this.add(v);
  }
}

function add(a, b) {
    return new Vector(a.x + b.x, a.y + b.y);
};

function sub(a, b) {
    return new Vector(a.x - b.x, a.y - b.y);
};

function scale(v, s) {
    return v.clone().scale(s);
};

function randomVector() {
    return new Vector(
        Math.random() * 2 - 1,
        Math.random() * 2 - 1
    );
};

function unitVector(radian: number) {
  return new Vector(Math.cos(radian), Math.sin(radian));
}

function randomNum(from, to) {
  const size = to - from;
  return (Math.random() * size) + from;
}

function randomInt(from, to) {
  const size = to - from;
  return Math.ceil((Math.random() * size) + from);
}

main();

/**
 * requestAnimationFrame
 */
window.requestAnimationFrame = (function(){
    return  window.requestAnimationFrame       ||
            window.webkitRequestAnimationFrame ||
            (window as any).mozRequestAnimationFrame    ||
            (window as any).oRequestAnimationFrame      ||
            (window as any).msRequestAnimationFrame     ||
            function (callback) {
                window.setTimeout(callback, 1000 / 60);
            };
})();



/**
 * GravityPoint
 */
function GravityPoint(x, y, radius, targets) {
    Vector.call(this, x, y);
    this.radius = radius;
    this.currentRadius = radius * 0.5;

    this._targets = {
        particles: targets.particles || [],
        gravities: targets.gravities || []
    };
    this._speed = new Vector();
}

GravityPoint.RADIUS_LIMIT = 65;
GravityPoint.interferenceToPoint = true;

GravityPoint.prototype = (function(o) {
    var s = new Vector(0, 0), p;
    for (p in o) s[p] = o[p];
    return s;
})({
    gravity:       0.05,
    isMouseOver:   false,
    dragging:      false,
    destroyed:     false,
    _easeRadius:   0,
    _dragDistance: null,
    _collapsing:   false,

    hitTest: function(p) {
        return this.distanceTo(p) < this.radius;
    },

    startDrag: function(dragStartPoint) {
        this._dragDistance = sub(dragStartPoint, this);
        this.dragging = true;
    },

    drag: function(dragToPoint) {
        this.x = dragToPoint.x - this._dragDistance.x;
        this.y = dragToPoint.y - this._dragDistance.y;
    },

    endDrag: function() {
        this._dragDistance = null;
        this.dragging = false;
    },

    addSpeed: function(d) {
        this._speed = this._speed.add(d);
    },

    collapse: function(e) {
        this.currentRadius *= 1.75;
        this._collapsing = true;
    },

    render: function(ctx) {
        if (this.destroyed) return;

        var particles = this._targets.particles,
            i, len;

        for (i = 0, len = particles.length; i < len; i++) {
            particles[i].addSpeed(sub(this, particles[i]).normalize().scale(this.gravity));
        }

        this._easeRadius = (this._easeRadius + (this.radius - this.currentRadius) * 0.07) * 0.95;
        console.log('4 _easeRadius', this._easeRadius);
        this.currentRadius += this._easeRadius;
        if (this.currentRadius < 0) this.currentRadius = 0;

        if (this._collapsing) {
            this.radius *= 0.75;
            if (this.currentRadius < 1) this.destroyed = true;
            this._draw(ctx);
            return;
        }

        var gravities = this._targets.gravities,
            g, absorp,
            area = this.radius * this.radius * Math.PI, garea;

        for (i = 0, len = gravities.length; i < len; i++) {
            g = gravities[i];

            if (g === this || g.destroyed) continue;

            if (
                (this.currentRadius >= g.radius || this.dragging) &&
                this.distanceTo(g) < (this.currentRadius + g.radius) * 0.85
            ) {
                g.destroyed = true;
                this.gravity += g.gravity;

                absorp = sub(g, this).scale(g.radius / this.radius * 0.5);
                this.addSpeed(absorp);

                console.log('1 current radius', this.currentRadius);
                console.log('1 radius', this.radius);
                garea = g.radius * g.radius * Math.PI;
                this.currentRadius = Math.sqrt((area + garea * 3) / Math.PI);
                this.radius = Math.sqrt((area + garea) / Math.PI);
                console.log('2 current radius', this.currentRadius);
                console.log('2 radius', this.radius);
            }

            g.addSpeed(sub(this, g).normalize().scale(this.gravity));
        }

        if (GravityPoint.interferenceToPoint && !this.dragging)
            this.add(this._speed);

        this._speed = new Vector();

        if (this.currentRadius > GravityPoint.RADIUS_LIMIT) this.collapse();

        this._draw(ctx);
    },

    _draw: function(ctx) {
        var grd, r;

        ctx.save();

        grd = ctx.createRadialGradient(this.x, this.y, this.radius, this.x, this.y, this.radius * 5);
        grd.addColorStop(0, 'rgba(0, 0, 0, 0.1)');
        grd.addColorStop(1, 'rgba(0, 0, 0, 0)');
        ctx.beginPath();
        ctx.arc(this.x, this.y, this.radius * 5, 0, Math.PI * 2, false);
        ctx.fillStyle = grd;
        ctx.fill();

        r = Math.random() * this.currentRadius * 0.7 + this.currentRadius * 0.3;
        // console.log('3 this.currentRadius', this.currentRadius);
        grd = ctx.createRadialGradient(this.x, this.y, r, this.x, this.y, this.currentRadius);
        grd.addColorStop(0, 'rgba(0, 0, 0, 1)');
        grd.addColorStop(1, Math.random() < 0.2 ? 'rgba(255, 196, 0, 0.15)' : 'rgba(103, 181, 191, 0.75)');
        ctx.beginPath();
        ctx.arc(this.x, this.y, this.currentRadius, 0, Math.PI * 2, false);
        ctx.fillStyle = grd;
        ctx.fill();
        ctx.restore();
    }
});


/**
 * Particle
 */
function Particle(x, y, radius) {
    Vector.call(this, x, y);
    this.radius = radius;

    this._latest = new Vector();
    this._speed  = new Vector();
}

Particle.prototype = (function(o) {
    var s = new Vector(0, 0), p;
    for (p in o) s[p] = o[p];
    return s;
})({
    addSpeed: function(d) {
        this._speed.add(d);
    },

    update: function() {
        if (this._speed.length() > 12) this._speed.normalize().scale(12);

        this._latest.set(this);
        this.add(this._speed);
    }

    // render: function(ctx) {
    //     if (this._speed.length() > 12) this._speed.normalize().scale(12);

    //     this._latest.set(this);
    //     this.add(this._speed);

    //     ctx.save();
    //     ctx.fillStyle = ctx.strokeStyle = '#fff';
    //     ctx.lineCap = ctx.lineJoin = 'round';
    //     ctx.lineWidth = this.radius * 2;
    //     ctx.beginPath();
    //     ctx.moveTo(this.x, this.y);
    //     ctx.lineTo(this._latest.x, this._latest.y);
    //     ctx.stroke();
    //     ctx.beginPath();
    //     ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2, false);
    //     ctx.fill();
    //     ctx.restore();
    // }
});



// Initialize

function f() {

    // Configs

    var BACKGROUND_COLOR      = 'rgba(11, 51, 56, 1)',
        PARTICLE_RADIUS       = 1,
        G_POINT_RADIUS        = 10,
        G_POINT_RADIUS_LIMITS = 65;


    // Vars

    var canvas, context,
        bufferCvs, bufferCtx,
        screenWidth, screenHeight,
        mouse = new Vector(),
        gravities: any = [],
        particles: any = [],
        grad,
        gui, control;


    // Event Listeners

    function resize(e) {
        screenWidth  = canvas.width  = window.innerWidth;
        screenHeight = canvas.height = window.innerHeight;
        bufferCvs.width  = screenWidth;
        bufferCvs.height = screenHeight;
        context   = canvas.getContext('2d');
        bufferCtx = bufferCvs.getContext('2d');

        var cx = canvas.width * 0.5,
            cy = canvas.height * 0.5;

        grad = context.createRadialGradient(cx, cy, 0, cx, cy, Math.sqrt(cx * cx + cy * cy));
        grad.addColorStop(0, 'rgba(0, 0, 0, 0)');
        grad.addColorStop(1, 'rgba(0, 0, 0, 0.35)');
    }

    function mouseMove(e) {
        mouse.set(e.clientX, e.clientY);

        var i, g, hit = false;
        for (i = gravities.length - 1; i >= 0; i--) {
            g = gravities[i];
            if ((!hit && g.hitTest(mouse)) || g.dragging)
                g.isMouseOver = hit = true;
            else
                g.isMouseOver = false;
        }

        canvas.style.cursor = hit ? 'pointer' : 'default';
    }

    function mouseDown(e) {
        for (var i = gravities.length - 1; i >= 0; i--) {
            if (gravities[i].isMouseOver) {
                gravities[i].startDrag(mouse);
                return;
            }
        }
        gravities.push(new GravityPoint(e.clientX, e.clientY, G_POINT_RADIUS, {
            particles: particles,
            gravities: gravities
        }));
    }

    function mouseUp(e) {
        for (var i = 0, len = gravities.length; i < len; i++) {
            if (gravities[i].dragging) {
                gravities[i].endDrag();
                break;
            }
        }
    }

    function doubleClick(e) {
        for (var i = gravities.length - 1; i >= 0; i--) {
            if (gravities[i].isMouseOver) {
                gravities[i].collapse();
                break;
            }
        }
    }


    // Functions

    function addParticle(num) {
        var i, p;
        for (i = 0; i < num; i++) {
            p = new Particle(
                Math.floor(Math.random() * screenWidth - PARTICLE_RADIUS * 2) + 1 + PARTICLE_RADIUS,
                Math.floor(Math.random() * screenHeight - PARTICLE_RADIUS * 2) + 1 + PARTICLE_RADIUS,
                PARTICLE_RADIUS
            );
            p.addSpeed(randomVector());
            particles.push(p);
        }
    }

    function removeParticle(num) {
        if (particles.length < num) num = particles.length;
        for (var i = 0; i < num; i++) {
            particles.pop();
        }
    }


    // GUI Control

    control = {
        particleNum: 10
    };


    // Init

    canvas  = document.getElementById('c');
    bufferCvs = document.createElement('canvas');

    window.addEventListener('resize', resize, false);
    resize(null);

    addParticle(control.particleNum);

    canvas.addEventListener('mousemove', mouseMove, false);
    canvas.addEventListener('mousedown', mouseDown, false);
    canvas.addEventListener('mouseup', mouseUp, false);
    canvas.addEventListener('dblclick', doubleClick, false);


    // Start Update
    var throttleCount = -1;
    var throttleMax = 100;
    var loop = function() {
        var i, len, g, p;

        context.save();
        context.fillStyle = BACKGROUND_COLOR;
        context.fillRect(0, 0, screenWidth, screenHeight);
        context.fillStyle = grad;
        context.fillRect(0, 0, screenWidth, screenHeight);
        context.restore();

        for (i = 0, len = gravities.length; i < len; i++) {
            g = gravities[i];
            if (g.dragging) g.drag(mouse);
            g.render(context);
            if (g.destroyed) {
                gravities.splice(i, 1);
                len--;
                i--;
            }
        }
      
        bufferCtx.save();
        bufferCtx.globalCompositeOperation = 'destination-out';
        bufferCtx.globalAlpha = 0.35;
        bufferCtx.fillRect(0, 0, screenWidth, screenHeight);
        bufferCtx.restore();

        // パーティクルをバッファに描画
        // for (i = 0, len = particles.length; i < len; i++) {
        //     particles[i].render(bufferCtx);
        // }
        len = particles.length;
        bufferCtx.save();
        bufferCtx.fillStyle = bufferCtx.strokeStyle = '#fff';
        bufferCtx.lineCap = bufferCtx.lineJoin = 'round';
        bufferCtx.lineWidth = PARTICLE_RADIUS * 2;
        bufferCtx.beginPath();
        for (i = 0; i < len; i++) {
            p = particles[i];
            p.update();
            bufferCtx.moveTo(p.x, p.y);
            bufferCtx.lineTo(p._latest.x, p._latest.y);
        }
        bufferCtx.stroke();
        bufferCtx.beginPath();
        for (i = 0; i < len; i++) {
            p = particles[i];
            bufferCtx.moveTo(p.x, p.y);
            bufferCtx.arc(p.x, p.y, p.radius, 0, Math.PI * 2, false);
        }
        bufferCtx.fill();
        bufferCtx.restore();

        // バッファをキャンバスに描画
        if (throttleCount < 0 || throttleCount > throttleMax) {
          context.drawImage(bufferCvs, 0, 0);
          throttleCount = 0;
        } else {
          throttleCount++;
        }

        requestAnimationFrame(loop);
    };
    // loop();

};



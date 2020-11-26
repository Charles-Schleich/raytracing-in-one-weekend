
# R(ust)ay Tracing in One  Weekend


I looked at the world and thought, 'Just one more ray tracer, and it will be complete'.
Here is my implementation of Peter Shirleys "Ray Tracing in One weekend" !<br/>
I have thoroughly enjoyed myself,
Here is a link to it https://raytracing.github.io/books/RayTracingInOneWeekend.html

  
### Build + Run

`cargo build --release`  <br  />

`cargo run --release > image.ppm` (i will make this better)

 <br  />

It uses Rayon for Data-Parallelism, this gets pretty significant speed ups the higher the core count of your CPU was.
A Random Scene Rending at 4k, on an 8 Core CPU, with 500 Samples Per pixel, and a 50 bounce limit took about 176 minutes to render.

I suppose my biggest regret is not doing this sooner.
(Such that this code could have join the many other ray tracers in the Artic Code vault, to be preserved for 100's of years)

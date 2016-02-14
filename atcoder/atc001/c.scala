final object FFT {
  import java.lang.Math.{cos, sin}
  sealed case class Comp(val x: Double, val y: Double) {
    def +(other: Comp): Comp = Comp(x + other.x, y + other.y)
    def -(other: Comp): Comp = Comp(x - other.x, y - other.y)
    def *(other: Comp): Comp = Comp(x * other.x - y * other.y, x * other.y + y * other.x)
    def /(real: Double): Comp = Comp(x / real, y / real)
    override def toString: String = x + " + " + y + "i"
  }
  private[this] val pi = 3.141592653589793238463
  private[this] def inplace_internal_fft(
		       f: Array[Comp], //const
		       output: Array[Comp], //mutable
		       ztbl: Array[Comp], // const
		       x: Int,
		       fstart: Int,
		       fstep: Int,
		       n: Int,
		       ostart: Int) {
    if (n == 1) {
      output(ostart) = f(fstart)
    } else {
      inplace_internal_fft(f, output, ztbl, x + 1,
			 fstart, 2 * fstep, n / 2, ostart)
      inplace_internal_fft(f, output, ztbl, x + 1,
			 fstart + fstep, 2 * fstep, n / 2, ostart + n / 2)
      val zeta = ztbl(x)
      var pzeta = Comp(1, 0)
      for (i <- 0 until (n / 2)) {
        val f0 = output(ostart + i)
        val f1 = output(ostart + i + n / 2)
        output(ostart + i) = f0 + pzeta * f1
        output(ostart + i + n / 2) = f0 - pzeta * f1
        pzeta *= zeta
      }
    }
  }
  def ceil_pow2(n: Int): Int = {
    var t = n
    while ((t & (t - 1)) != 0) {
      t += t & (-t)
    }
    t
  }
  def transform(f: Array[Comp], n: Int): Array[Comp] = {
    val p = Integer.bitCount(n - 1) // n = 2^p
    val ztbl = Array.fill(p)(Comp(0,0))
    for (i <- 0 until p) {
      val d = n >> i
      val zeta = Comp(cos(2 * pi / d), sin(2 * pi / d))
      ztbl(i) = zeta
    }
    val output = Array.fill(n)(Comp(0,0))
    inplace_internal_fft(f, output, ztbl, 0, 0, 1, n, 0)
    output
  }
 
  def inverse_transform(f: Array[Comp], n: Int): Array[Comp] = {
    val p = Integer.bitCount(n - 1) // n = 2^p
    val ztbl = Array.fill(p)(Comp(0,0))
    for (i <- 0 until p) {
      val d = n >> i;
      val zeta = Comp(cos(2 * pi / d), - sin(2 * pi / d));
      ztbl(i) = zeta;
    }
    val output = Array.fill(n)(Comp(0,0))
    inplace_internal_fft(f, output, ztbl, 0, 0, 1, n, 0)
    output.map(_ / n)
  }
}


/*
 * Reference: http://lizan.asia/blog/2012/12/11/scala-competitive/
 */

object Main extends App {
  import java.{util => ju}
  import scala.annotation._
  import scala.collection._
  import scala.collection.{mutable => mu}
  import scala.collection.JavaConverters._
  import scala.math._
  import FFT.Comp

  val sc = new ju.Scanner(System.in)
  val orign = sc.nextInt
  val n = FFT.ceil_pow2(orign) * 2
  val ab = Array.fill(orign)((sc.nextDouble, sc.nextDouble))
  var ca, cb = Array.fill(n)(Comp(0,0))
  for (i <- 0 until orign) {
    ca(i) = Comp(ab(i)._1, 0)
    cb(i) = Comp(ab(i)._2, 0)
  }
  ca = FFT.transform(ca, n)
  cb = FFT.transform(cb, n)
  for (i <- 0 until n) ca(i) *= cb(i)
  ca = FFT.inverse_transform(ca, n)
  println(0)
  for (i <- 0 until (2 * orign - 1)) {
    println((ca(i).x + 0.5).asInstanceOf[Long])
  }
}
final object AllFactors {
  def apply(a: Long): Array[Long] = {
    val buf = scala.collection.mutable.ArrayBuffer[Long]()
    var p: Long = 1
    while (p * p < a) {
      if (a % p == 0) {
        buf += p
      }
      p += 1
    }
    if (p * p == a) {
      buf ++= Array(p) ++ buf.toArray.reverse.map(a / _)
    } else {
      buf ++= buf.toArray.reverse.map(a / _)
    }
    buf.toArray
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

  val sc = new ju.Scanner(System.in)
  val n = sc.nextLong
  val s = AllFactors(n).sum
  println((n * 2) compareTo s match {
    case 0 => "Perfect"
    case x if x < 0 => "Abundant"
    case _ => "Deficient"
  })
}
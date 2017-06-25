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
  val n = sc.nextInt
  val s = Array.fill(n)(sc.nextInt)
  val tot = s.sum
  val m = sc.nextInt
  val know = Array.tabulate(n)(i => Array.tabulate(n)(i == _))
  (0 until m) foreach { _ =>
    val a = sc.nextInt
    val b,c = sc.nextInt - 1
    a match {
      case 0 => know(b)(c) = true
      case 1 => know(b)(c) match {
        case true => println(s(c) + " " + s(c))
        case false => {
          val ptot = (0 until n).filter(know(b)).map(s).sum
          val indet = know(b).count(!_)
          println((0 max (tot - ptot - 100 * (indet - 1))) + " " + (100 min (tot - ptot)))
        }
      }
    }
  }
}
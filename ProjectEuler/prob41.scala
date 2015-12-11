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
  @tailrec
  def digits(v:Int, acc: Int = 0): Int = {
    v match {
      case 0 => acc
      case _ => digits(v / 10, acc | 1 << (v % 10))
    }
  }
  def isPrime(v : Int) = {
    v >= 2 && (2 to Math.sqrt(v).asInstanceOf[Int]).forall(v % _ != 0)
  }
  val sc = new ju.Scanner(System.in)
  val pow10 = Array(1,10,100,1000,10000,100000,1000000,10000000,100000000)
  var sols = new mu.ListBuffer[Int]
  (1 to 7) foreach { n =>
    (pow10(n - 1) until pow10(n)) foreach { i =>
      if (digits(i) == (2 << n) - 2 && isPrime(i)) sols += i
    }
  }
  println(sols.max)
}
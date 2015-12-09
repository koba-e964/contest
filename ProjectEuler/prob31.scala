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
  final val coins = Array(2,5,10,20,50,100,200)
  final def calc(rem: Int, pos: Int): Long = {
    pos match {
      case -1 => 1
      case _ => (0 to rem / coins(pos)).map{x => calc(rem - (x : Int) * coins(pos), pos - 1)}.sum
    }
  }
  println(calc(200, coins.length - 1))
}
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
  def dir(deg : Int) : String = {
    def p4(a: String, b: String, c : String) = Array(a, a + a + b, a + b, b + a + b, b, b + c + b, c + b, c + c + b)
    val t = p4("N", "E", "S") ++ p4("S", "W", "N")
    val i = ((deg - 113 + 225) % 3600) / 225 
    t(i)
  }
  def str(dis: Int) : Int = {
    val m = (dis + 3)
    val t = Array(0, 3, 16, 34, 55, 80, 108, 139, 172, 208, 245, 285, 327)
    t.count(_ * 6 <= m) - 1
  }
  val deg = sc.nextInt
  val dis = sc.nextInt
  str(dis) match {
    case 0 => println("C 0")
    case s => println(dir(deg) + " " + s)
  }
}
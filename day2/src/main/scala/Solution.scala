import java.io._
import java.math._
import java.security._
import java.text._
import java.util._
import java.util.concurrent._
import java.util.function._
import java.util.regex._
import java.util.stream._

object Solution {

  // Complete the solve function below.
  def solve(meal_cost: Double, tip_percent: Int, tax_percent: Int) = {
    val tip = meal_cost * tip_percent / 100.0
    val tax = meal_cost * tax_percent / 100.0

    Math.round (meal_cost + tip + tax)
  }

  def main(args: Array[String]) {
    val stdin = scala.io.StdIn

    val meal_cost = stdin.readLine.trim.toDouble

    val tip_percent = stdin.readLine.trim.toInt

    val tax_percent = stdin.readLine.trim.toInt

    println(solve(meal_cost, tip_percent, tax_percent))
  }
}

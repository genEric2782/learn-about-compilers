import Dependencies._

ThisBuild / scalaVersion     := "2.13.16"
ThisBuild / version          := "0.1.0-SNAPSHOT"
ThisBuild / organization     := "com.example"
ThisBuild / organizationName := "example"

lazy val root = (project in file("."))
  .settings(
    name := "learn-about-compilers-scala",
    libraryDependencies ++= Seq(
      munit % Test,
      "com.lihaoyi" %% "upickle" % "3.3.1",
      "com.lihaoyi" %% "os-lib" % "0.10.0"
    )
  )

// See https://www.scala-sbt.org/1.x/docs/Using-Sonatype.html for instructions on how to publish to Sonatype.


package com.demo;

public class Task {
  public String name;
  public boolean complete;

  public Task(String n) {
    name = n;
    complete = false;
  }

  public Task(String n, boolean c) {
    name = n;
    complete = c;
  }
}

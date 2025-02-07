package com.demo.rest;

import java.util.ArrayList;

import com.demo.Task;

import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.MediaType;

@Path("tasks")
public class TasksResource {

  @GET
  @Produces(MediaType.APPLICATION_JSON)
  public ArrayList<Task> getTasks() {
    ArrayList<Task> tasks = new ArrayList<>();
    tasks.add(new Task("Create OpenLiberty Starter App", true));
    tasks.add(new Task("Add react frontend project to openliberty app", true));
    tasks.add(new Task("Include bootstrap in project setup", true));
    tasks.add(new Task("Demo todolist demo for stakeholders", false));
    return tasks;
  }
}

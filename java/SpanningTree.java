package main.java.functions;

import java.util.ArrayList;
import java.util.Collection;
import main.java.types.Restaurant;
import main.java.types.Street;

public class SpanningTree {
  private int searchId;

  public SpanningTree(Restaurant res) {
    Node root = new Node(res);

  }

  private class Node {
    private Restaurant restaurant;
    public Collection<Node> connections;

    public Node(Restaurant r) {
      restaurant = r;
    }
  }

  private class NodeQueue {
    private ArrayList<Node> stack;

    public void push(Node e) {
      stack.add(e);
    }

    public void pushAll(Collection<Node> es) {
      stack.addAll(es);
    }

    public Node pop() {
      if (!stack.isEmpty()) {
        Node node = stack.get(0);
        stack.remove(0);
        return node;
      }
      return null;
    }

    public boolean isEmpty() {
      return stack.isEmpty();
    }
  }

  // public ArrayList<Node> findPath(Node src, Node dest) {
  // ArrayList<Node> history = new ArrayList<>();

  // NodeQueue queue = new NodeQueue();
  // queue.push(src);
  // while (!queue.isEmpty()) {
  // Node currentNode = queue.pop();
  // for (Node e : currentNode.connections) {
  // if (currentNode == dest) {
  // // TODO: Return ArrayList that is the path
  // return history;
  // }
  // queue.pushAll(e.connections);
  // }
  // }

  // searchId++;
  // return null;
  // }
}

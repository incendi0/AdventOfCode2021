package qzem.leviosa.aoc;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

class Tree {
    NestedNode root;

    public Tree() {
    }

    public Tree(String s) {
        root = new NestedNode(s, 0);
        calculateDepth(root);
    }

    public static Tree add(Tree lhs, Tree rhs) {
        Tree ret = new Tree();
        ret.root = new NestedNode();
        ret.root.left = lhs.root;
        ret.root.right = rhs.root;
        lhs.root.parent = ret.root;
        rhs.root.parent = ret.root;
        calculateDepth(ret.root);
        ret.regular();
        return ret;
    }

    private static void calculateDepth(NestedNode root) {
        int depth = 0;
        Queue<Node> q = new LinkedList<>();
        q.offer(root);
        while (!q.isEmpty()) {
            int size = q.size();
            for (int i = 0; i < size; i++) {
                Node t = q.poll();
                t.depth = depth;
                if (t.left != null) {
                    q.offer(t.left);
                }
                if (t.right != null) {
                    q.offer(t.right);
                }
            }
            depth += 1;
        }
    }

    public void regular() {
//        System.out.println("before regular" + root);
        int count = 0;
        while (true) {
            if (explodeOnce()) {
//                System.out.println("explodeOnce" + root);
            } else if (splitOnce()) {
//                System.out.println("splitOnce" + root);
            } else {
                break;
            }
            count += 1;
//            System.out.println("第" + count + "次： " + root);
        }
//        System.out.println("after regular" + root);
    }

    public int magnitude(Node root) {
        if (root instanceof IntNode) {
            return ((IntNode) root).value;
        } else {
            int l = magnitude(root.left);
            int r = magnitude(root.right);
            return l * 3 + r * 2;
        }
    }

    private boolean explodeOnce() {
        List<Node> stack = new ArrayList<>();
        Node curr = root;
        while (!stack.isEmpty() || curr != null) {
            while (curr != null) {
                stack.add(curr);
                curr = curr.left;
            }
            curr = stack.remove(stack.size() - 1);
            Node t = curr;
            if (t instanceof IntNode && t.depth > 4 && t.parent.right instanceof IntNode && t.parent.left instanceof IntNode) {
                Node explode = t.parent;
                IntNode lhs = findLeftNode(explode);
                if (lhs != null) {
                    lhs.value += ((IntNode) explode.left).value;
                }
                IntNode rhs = findRightNode(explode);
                if (rhs != null) {
                    rhs.value += ((IntNode) explode.right).value;
                }
                Node parent = explode.parent;
                boolean isLeft = explode == parent.left;
                IntNode newNode = new IntNode();
                newNode.value = 0;
                newNode.depth = parent.depth + 1;
                newNode.parent = parent;
                if (isLeft) {
                    parent.left = newNode;
                } else {
                    parent.right = newNode;
                }
                return true;
            }
            curr = curr.right;
        }
        return false;
    }

    private IntNode findLeftNode(Node explode) {
        Node curr = explode;
        Node parent = explode.parent;
        while (parent != null) {
            if (parent.left == curr) {
                curr = parent;
                parent = parent.parent;
            } else {
                Node ret = parent.left;
                while (ret.right != null) {
                    ret = ret.right;
                }
                if (!(ret instanceof IntNode)) {
                    throw new RuntimeException("corrupted tree structure");
                }
                return (IntNode) ret;
            }
        }
        return null;
    }

    private IntNode findRightNode(Node explode) {
        Node curr = explode;
        Node parent = explode.parent;
        while (parent != null) {
            if (parent.right == curr) {
                curr = parent;
                parent = parent.parent;
            } else {
                Node ret = parent.right;
                while (ret.left != null) {
                    ret = ret.left;
                }
                if (!(ret instanceof IntNode)) {
                    throw new RuntimeException("corrupted tree structure");
                }
                return (IntNode) ret;
            }
        }
        return null;
    }

    private boolean splitOnce() {
        List<Node> stack = new ArrayList<>();
        Node curr = root;
        while (!stack.isEmpty() || curr != null) {
            while (curr != null) {
                stack.add(curr);
                curr = curr.left;
            }
            curr = stack.remove(stack.size() - 1);
            Node t = curr;
            if (t instanceof IntNode && ((IntNode) t).value >= 10) {
                NestedNode newNode = new NestedNode();
                newNode.depth = t.depth;
                int v = ((IntNode) t).value;
                newNode.left = new IntNode(v / 2);
                newNode.left.depth = t.depth + 1;
                newNode.left.parent = newNode;
                newNode.right = new IntNode(v / 2 + v % 2);
                newNode.right.depth = t.depth + 1;
                newNode.right.parent = newNode;
                newNode.parent = t.parent;
                boolean isLeft = t.parent.left == t;
                if (isLeft) {
                    t.parent.left = newNode;
                } else {
                    t.parent.right = newNode;
                }
                return true;
            }
            curr = curr.right;
        }
        return false;
    }

    @Override
    public String toString() {
        return "Tree{" +
                "root=" + root +
                '}';
    }
}

abstract class Node {
    Node parent;
    Node left, right;
    int depth;
    int len;
    int idx;
    String s;

    public Node() {
    }

    protected Node(String s, int idx) {
        this.idx = idx;
        this.s = s;

        do_parse();
    }

    abstract void do_parse();
}

class IntNode extends Node {
    int value;

    public IntNode() {
    }

    IntNode(String s, int idx) {
        super(s, idx);
    }

    public IntNode(int value) {
        this.value = value;
    }

    @Override
    void do_parse() {
        value = 0;
        int start = idx;
        while (idx < s.length() && Character.isDigit(s.charAt(idx))) {
            value = value * 10 + s.charAt(idx) - '0';
            idx += 1;
        }
        len = idx - start;
    }

    @Override
    public String toString() {
        return "IntNode{" +
                "depth=" + depth +
                ", value=" + value +
                '}';
    }
}

class NestedNode extends Node {

    public NestedNode() {
    }

    NestedNode(String s, int idx) {
        super(s, idx);
    }

    @Override
    void do_parse() {
        int start = idx;
        idx += 1; // [
        left = s.charAt(idx) == '[' ? do_parse_nested() : do_parse_int();
        idx += left.len;
        idx += 1; // ,
        right = s.charAt(idx) == '[' ? do_parse_nested() : do_parse_int();
        idx += right.len;
        idx += 1; // ]
        len = idx - start;
        left.parent = this;
        right.parent = this;
    }

    private NestedNode do_parse_nested() {
        return new NestedNode(s, idx);
    }

    private IntNode do_parse_int() {
        return new IntNode(s, idx);
    }

    @Override
    public String toString() {
        return "NestedNode{" +
                "left=" + left +
                ", right=" + right +
                '}';
    }
}

public class Day18 {
    public static void main(String[] args) {
//        String s = "[1,2]\n" +
//                "[[1,2],3]\n" +
//                "[9,[8,7]]\n" +
//                "[[1,9],[8,5]]\n" +
//                "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]\n" +
//                "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]\n" +
//                "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]\n" +
//                "[[[[[9,8],1],2],3],4]";
//        for (String exp : s.split("\n")) {
//            Tree t = new Tree(exp);
//            System.out.println(t);
//            System.out.println(t.magnitude(t.root));
//        }

//        String[] xs = {
//                "[[1,2],[[3,4],5]]",
//                "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
//                "[[[[1,1],[2,2]],[3,3]],[4,4]]",
//                "[[[[3,0],[5,3]],[4,4]],[5,5]]",
//                "[[[[5,0],[7,4]],[5,5]],[6,6]]",
//                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
//        };
//        for (String exp : xs) {
//            Tree t = new Tree(exp);
//            System.out.println(t);
//            System.out.println(t.magnitude(t.root));
//        }
//
//        String[] ex1 = {
//                "[[[[4,3],4],4],[7,[[8,4],9]]]",
//                "[1,1]",
//        };
//        Tree tr1 = new Tree(ex1[0]);
//        Tree tr2 = new Tree(ex1[1]);
//        Tree sum = Tree.add(tr1, tr2);

//        String toAdd1 = "[1,1]\n" +
//                "[2,2]\n" +
//                "[3,3]\n" +
//                "[4,4]\n" +
//                "[5,5]\n" +
//                "[6,6]";
//        Tree t = null;
//        for (String exp : toAdd1.split("\n")) {
//            if (t == null) {
//                t = new Tree(exp);
//            } else {
//                t = Tree.add(t, new Tree(exp));
//            }
//            System.out.println(t);
//        }
//        System.out.println(t.magnitude(t.root));


//        String toAdd = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n" +
//                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n" +
//                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n" +
//                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n" +
//                "[7,[5,[[3,8],[1,4]]]]\n" +
//                "[[2,[2,2]],[8,[8,1]]]\n" +
//                "[2,9]\n" +
//                "[1,[[[9,3],9],[[9,0],[0,7]]]]\n" +
//                "[[[5,[7,4]],7],1]\n" +
//                "[[[[4,2],2],6],[8,7]]\n";
//        Tree t = null;
//        for (String exp : toAdd.split("\n")) {
//            if (t == null) {
//                t = new Tree(exp);
//            } else {
//                t = Tree.add(t, new Tree(exp));
//            }
//        }
//        System.out.println(t.magnitude(t.root));

        String toAdd = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n" +
                "[[[5,[2,8]],4],[5,[[9,9],0]]]\n" +
                "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n" +
                "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n" +
                "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n" +
                "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n" +
                "[[[[5,4],[7,7]],8],[[8,3],8]]\n" +
                "[[9,3],[[9,9],[6,[4,9]]]]\n" +
                "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n" +
                "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]\n";
        Tree t = null;
        for (String exp : toAdd.split("\n")) {
            if (t == null) {
                t = new Tree(exp);
            } else {
                t = Tree.add(t, new Tree(exp));
            }
        }
        System.out.println(t.magnitude(t.root));

//        String[] xs = toAdd.split("\n");
//        int retXs = 0;
//        for (int i = 0; i < xs.length; i++) {
//            for (int j = 0; j < xs.length; j++) {
//                if (i != j) {
//                    Tree ti = new Tree(xs[i]);
//                    Tree tj = new Tree(xs[j]);
//                    Tree root = Tree.add(ti, tj);
//                    retXs = Math.max(retXs, root.magnitude(root.root));
//                }
//            }
//        }
//        System.out.println("retXs: " + retXs);

        String toAddPartOne = "[[[[7,1],[0,0]],[6,[8,2]]],[8,[3,8]]]\n" +
                "[[[3,6],[9,4]],[[[5,9],5],[8,0]]]\n" +
                "[[[2,2],2],[1,[[1,6],7]]]\n" +
                "[[[[0,9],7],[[3,2],8]],[6,[7,9]]]\n" +
                "[[[[4,1],6],[[7,6],[2,2]]],[[[1,1],9],4]]\n" +
                "[[[8,[3,7]],3],[[4,4],[[9,1],[3,5]]]]\n" +
                "[[4,[8,2]],[1,[0,5]]]\n" +
                "[8,[8,7]]\n" +
                "[[[[2,2],7],[3,[4,5]]],[[4,6],[[2,5],4]]]\n" +
                "[[[5,5],[[5,1],3]],[[2,[8,2]],[[6,9],[1,5]]]]\n" +
                "[0,7]\n" +
                "[[[[5,1],3],[8,[5,3]]],7]\n" +
                "[[5,[2,[0,6]]],[[[5,5],2],[9,[8,0]]]]\n" +
                "[[[[3,4],2],0],4]\n" +
                "[[[[5,3],[2,7]],6],[[4,0],[9,[7,2]]]]\n" +
                "[[[3,[2,5]],[3,3]],7]\n" +
                "[[[[5,1],1],[4,8]],[[5,[8,3]],2]]\n" +
                "[[4,[[8,1],[8,5]]],[[[4,1],0],6]]\n" +
                "[[[5,5],[5,9]],[0,[[6,8],[0,1]]]]\n" +
                "[4,[[[7,9],4],0]]\n" +
                "[[[[0,1],7],[[3,6],5]],[8,[5,[6,1]]]]\n" +
                "[[[7,7],[8,0]],[6,[8,[7,9]]]]\n" +
                "[[[9,2],1],6]\n" +
                "[[[4,4],[2,[5,0]]],[[[2,6],6],[5,[4,3]]]]\n" +
                "[[2,[[4,7],5]],1]\n" +
                "[[8,7],[[[2,0],7],[1,[0,3]]]]\n" +
                "[[9,[[9,3],[9,5]]],[[8,7],[[4,1],[6,5]]]]\n" +
                "[[3,4],[[9,4],5]]\n" +
                "[[5,[[8,3],5]],1]\n" +
                "[[0,[[9,0],[3,2]]],[2,[7,[5,1]]]]\n" +
                "[[9,[[9,5],[8,6]]],[[4,4],[[3,8],[1,6]]]]\n" +
                "[[[1,[5,2]],9],[[4,6],[3,[8,0]]]]\n" +
                "[[1,7],[[1,7],9]]\n" +
                "[[[[3,4],3],[[7,5],[9,1]]],[[[5,0],[3,0]],[[7,9],6]]]\n" +
                "[[[7,2],[[1,0],[5,6]]],[[[3,7],[8,9]],6]]\n" +
                "[[[[1,1],1],[[8,6],[9,8]]],[[[1,8],4],[8,9]]]\n" +
                "[[[8,9],0],3]\n" +
                "[[[1,7],[1,[3,9]]],[6,[0,[8,5]]]]\n" +
                "[[0,5],[6,5]]\n" +
                "[[[[6,8],[4,5]],[[7,4],6]],[[3,6],5]]\n" +
                "[[8,[[0,9],8]],[9,[7,[7,9]]]]\n" +
                "[0,[[[7,1],2],[[0,4],4]]]\n" +
                "[[0,[[9,1],5]],[1,4]]\n" +
                "[3,4]\n" +
                "[[[9,3],[1,3]],[[[4,8],3],[[1,3],[9,0]]]]\n" +
                "[[[[5,1],7],[[9,2],8]],[[[6,8],[5,4]],[0,1]]]\n" +
                "[8,[[1,[3,0]],[[7,9],4]]]\n" +
                "[[[6,4],[[2,9],[9,0]]],[7,[[0,0],3]]]\n" +
                "[[3,[[9,6],6]],2]\n" +
                "[[5,[[3,1],[7,5]]],[[[6,7],9],[[4,6],[5,2]]]]\n" +
                "[[[4,[6,5]],8],[[6,[8,0]],[[9,3],3]]]\n" +
                "[[[[4,9],[2,8]],9],[[[5,0],0],[[3,4],[2,8]]]]\n" +
                "[[3,[7,1]],[9,[[1,8],7]]]\n" +
                "[[9,1],[0,[[0,7],[7,1]]]]\n" +
                "[[7,[0,[7,6]]],[[[5,3],1],[6,[4,5]]]]\n" +
                "[8,[[[2,1],[6,9]],[[3,3],[4,6]]]]\n" +
                "[0,[7,[3,0]]]\n" +
                "[[[[1,6],3],[5,[8,0]]],[[[6,6],7],1]]\n" +
                "[[[7,[8,3]],3],[[[2,8],5],[0,[9,5]]]]\n" +
                "[[[[5,1],4],[[1,2],1]],7]\n" +
                "[[[3,[7,5]],7],3]\n" +
                "[[9,[6,[1,1]]],[[[4,1],[2,2]],[[9,5],[7,7]]]]\n" +
                "[2,7]\n" +
                "[[[9,[8,6]],[[9,0],[6,5]]],[[[6,7],5],[[7,7],[2,3]]]]\n" +
                "[[[0,[6,4]],2],[4,[7,[7,5]]]]\n" +
                "[[[[6,1],[9,1]],[[6,1],9]],[[2,6],0]]\n" +
                "[[0,[[1,8],[3,5]]],[4,[[8,2],[4,2]]]]\n" +
                "[[[[9,3],[4,2]],2],[[[2,1],[7,1]],[4,8]]]\n" +
                "[[[3,[0,2]],3],8]\n" +
                "[[[4,[4,9]],9],[[[4,4],5],9]]\n" +
                "[[[[8,2],7],9],[[[1,0],[3,8]],[[7,7],0]]]\n" +
                "[[[3,2],[9,7]],[[9,[8,2]],[[5,5],3]]]\n" +
                "[[[7,[3,1]],[[8,3],1]],[[[8,6],[7,0]],4]]\n" +
                "[[9,[[9,1],5]],[[4,[1,1]],2]]\n" +
                "[[[[7,4],[0,3]],7],[8,[6,[3,3]]]]\n" +
                "[5,5]\n" +
                "[[6,7],[1,[7,[8,1]]]]\n" +
                "[[1,[0,4]],7]\n" +
                "[[[4,0],[[0,1],[2,2]]],[9,[[9,9],[3,0]]]]\n" +
                "[[[6,0],[[8,6],3]],[[5,1],[[8,1],[2,7]]]]\n" +
                "[[[[8,3],7],5],[9,[[5,1],8]]]\n" +
                "[[[[4,0],[5,2]],[[0,0],7]],2]\n" +
                "[[[[0,1],6],2],[[8,2],6]]\n" +
                "[[[[2,4],1],[[6,7],9]],[[[1,6],9],3]]\n" +
                "[[5,5],[[8,[7,7]],[5,8]]]\n" +
                "[[6,[[9,2],[9,7]]],[[[8,5],[4,4]],7]]\n" +
                "[[[9,[7,7]],[6,0]],[7,[[8,7],[1,2]]]]\n" +
                "[[7,[6,2]],[[9,[5,2]],[1,4]]]\n" +
                "[[[7,[5,9]],[[3,9],[4,5]]],[0,6]]\n" +
                "[[9,[8,[2,2]]],[[9,7],[1,1]]]\n" +
                "[[[[2,3],4],[[4,8],9]],[[9,[8,6]],[[0,9],0]]]\n" +
                "[[0,[[9,3],0]],[8,8]]\n" +
                "[[[[2,9],6],[[2,8],9]],[[[0,5],6],[[6,1],7]]]\n" +
                "[[9,[[8,3],[5,8]]],[[7,[3,0]],3]]\n" +
                "[[[4,[4,2]],0],1]\n" +
                "[[[[9,6],[5,8]],[6,2]],[[[8,0],[7,0]],[[5,6],4]]]\n" +
                "[[[8,0],[[4,3],[7,4]]],[[3,[7,9]],[[7,3],6]]]\n" +
                "[[3,[5,[0,3]]],[5,4]]\n" +
                "[[[[1,2],[6,3]],1],[[7,[5,2]],[[8,8],7]]]\n" +
                "[[4,[[8,0],[7,1]]],[[8,[8,0]],[[1,5],3]]]\n";
        Tree tt = null;
        for (String exp : toAddPartOne.split("\n")) {
            if (tt == null) {
                tt = new Tree(exp);
            } else {
                tt = Tree.add(tt, new Tree(exp));
            }
        }
        System.out.println(tt.magnitude(tt.root));

        String[] xs = toAddPartOne.split("\n");
        int retXs = 0;
        for (int i = 0; i < xs.length; i++) {
            for (int j = 0; j < xs.length; j++) {
                if (i != j) {
                    Tree ti = new Tree(xs[i]);
                    Tree tj = new Tree(xs[j]);
                    Tree root = Tree.add(ti, tj);
                    retXs = Math.max(retXs, root.magnitude(root.root));
                }
            }
        }
        System.out.println("retXs: " + retXs);
    }
}
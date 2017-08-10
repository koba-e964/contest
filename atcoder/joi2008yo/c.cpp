#include <iostream>
#include <set>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;


int main(void) {
  int n;
  cin >> n;
  set<int> a;
  REP(i, 0, n) {
    int v;
    cin >> v;
    a.insert(v);
  }
  set<int> b;
  REP(i, 1, 2 * n + 1) {
    if (a.count(i) == 0) {
      b.insert(i);
    }
  }
  int field = -1;
  int turn = 0;
  while (not a.empty() && not b.empty()) {
    set<int> *id;
    if (turn == 0) {
      id = &a;
    } else {
      id = &b;
    }
    set<int>::iterator it = id->lower_bound(field);
    if (it == id->end()) {
      field = -1;
      turn = 1 - turn;
      continue;
    }
    field = *it;
    id->erase(it);
    turn = 1 - turn;
  }
  cout << b.size() << endl << a.size() << endl;
}

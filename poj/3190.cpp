#include <algorithm>
#include <cassert>
#include <iostream>
#include <queue>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef pair<int, int> PI;
typedef pair<int, PI> PIPI;

int main(void){
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  vector<PI> t(n);
  vector<PIPI> events;
  REP(i, 0, n) {
    int x, y;
    cin >> x >> y;
    t[i] = PI(x, y);
    events.push_back(PIPI(x, PI(0, i)));
    events.push_back(PIPI(y + 1, PI(-1, i))); // If an eliminiation and an addition are performed at the same time, an elimination must be done first.
  }
  sort(events.begin(), events.end());
  int ma = 0;
  int cur = 0;
  // counts the maximum number of necessary stalls at each moment
  REP(i, 0, events.size()) {
    PIPI ev = events[i];
    switch(ev.second.first) {
    case 0: // adding
      cur++;
      break;
    case -1: // eliminating
      cur--;
      break;
    }
    assert (cur >= 0);
    ma = max(ma, cur);
  }
  cout << ma << endl;
  queue<int> vacant;
  REP(i, 0, ma) {
    vacant.push(i);
  }
  vector<bool> occupied(ma, false);
  vector<int> allocation(n, -1);
  REP(i, 0, events.size()) {
    PIPI ev = events[i];
    switch(ev.second.first) {
    case 0: {
      int q = vacant.front(); vacant.pop();
      occupied[q] = true;
      allocation[ev.second.second] = q;
    }
      break;
    case -1: {
      int q = allocation[ev.second.second];
      occupied[q] = false;
      vacant.push(q);
    }
    }
  }
  REP(i, 0, n) {
    cout << allocation[i] + 1 << endl;
  }
}

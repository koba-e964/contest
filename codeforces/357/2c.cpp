#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const string ins = "insert";
const string getm = "getMin";
const string rem = "removeMin";

int main(void){
  int n;
  cin >> n;
  priority_queue<int, vector<int>, greater<int> > que;
  vector<string> log;
  REP(i, 0, n) {
    string op;
    cin >> op;
    stringstream ss;
    if (op == ins) {
      int v;
      cin >> v;
      que.push(v);
      ss << ins << " " << v << endl;
      log.push_back(ss.str());
      continue;
    }
    if (op == getm) {
      int v;
      cin >> v;
      while (not que.empty() && que.top() < v) {
        que.top(); que.pop();
	ss << rem << endl;
	log.push_back(ss.str());
        ss.str("");
      }
      if (que.empty() || que.top() != v) {
	que.push(v);
	ss << ins << " " << v << endl;
	log.push_back(ss.str());
        ss.str("");
      }
      ss << getm << " " << v << endl;
      log.push_back(ss.str());
      continue;
    }
    assert(op == rem);
    if (que.empty()) {
      ss << ins << " " << 0 << endl;
      log.push_back(ss.str());
      ss.str("");
    } else {
      que.pop();
    }
    ss << rem << endl;
    log.push_back(ss.str());
  }
  cout << log.size() << endl;
  REP(i, 0, log.size()) {
    cout << log[i];
  }
}

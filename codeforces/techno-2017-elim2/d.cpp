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
const ll mod = 1e9 + 7;



int main(void){
  int n, a, b, k;
  cin >> n >> a >> b >> k;
  string s;
  cin >> s;
  VI hit;
  REP(i, 0, n) {
    if (s[i] == '1') {
      hit.push_back(i);
    }
  }
  VI bombard;
  REP(i, 0, k + 1) {
    int start = i == 0 ? 0 : hit[i - 1] + 1;
    int end = i == k ? n : hit[i];
    int q = (end - start) / b;
    int r = (end - start) % b;
    REP(j, 0, q) {
      bombard.push_back(start + j * b + r);
    }
  }
  REP(i, 0, a - 1) {
    bombard.pop_back();
  }
  cout << bombard.size() << endl;
  REP(i, 0, bombard.size()) {
    cout << bombard[i] + 1 << (i == bombard.size() - 1 ? "\n" : " ");
  }
}

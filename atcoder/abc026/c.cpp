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
typedef pair<int, int> PI;
const double EPS=1e-9;

VI child[30];



int b[30] = {-1};


int rec(int v) {
  if (child[v].size() == 0) {
    return 1;
  }
  int ma = 0, mi = 100000000;
  REP(i ,0,  child[v].size()) {
    int w = child[v][i];
    int r = rec(w);
    ma = max(ma, r);
    mi = min(mi, r);
  }
  return ma + mi + 1;
}


int main(void){
  int n;
  cin >> n;
  REP(i, 1, n) {
    cin >> b[i];
    b[i] --;
    child[b[i]].push_back(i);
  }
  cout << rec(0) << endl;
  
}

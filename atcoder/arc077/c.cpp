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
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  VL former, latter;
  for (int i = n - 1; i >= 0; i -= 2) {
    former.push_back(a[i]);
  }
  for (int i = n - 2; i >= 0; i -= 2) {
    latter.push_back(a[i]);
  }
  for (int i = latter.size() - 1; i >= 0; --i) {
    former.push_back(latter[i]);
  }
  REP(i, 0, former.size()) {
    cout << former[i] << (i == former.size() - 1 ? "\n": " ");
  }
}

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

void random_shuffle(vector<int> &vec) {
  int n = vec.size();
  REP (i, 0, n) {
    int r = rand() % (n - i);
    if (r > 0) {
      int tmp = vec[i + r];
      vec[i + r] = vec[i];
      vec[i] = tmp;
    }
  }
}

int main(void){
  int n;
  cin >> n;
  vector<int> a(n), b(n);
  REP(i,0,n) {
    cin >> a[i];
  }
  REP (i, 0, n) {
    cin >> b[i];
  }
  int wins = 0;
  const int num_trial = (int) 1e6;
  srand(time(0));
  REP (trial, 0, num_trial) {
    random_shuffle(b);
    int cnt = 0;
    REP (i, 0, n) {
      cnt += a[i] > b[i];
    }
    wins += cnt >= n / 2 + 1;
  }
  cout << (double) wins / (double)num_trial << endl;
}

#include <algorithm>
#include <iostream>
#include <sstream>
#include <random>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;

const int N = 312434;

struct mo_cmp {
  int b;
  mo_cmp(int b) : b(b) {}
  bool operator()(pair<pair<int, int>, int> x, pair<pair<int, int>, int> y) {
    pair<int, int> xp = x.first, yp = y.first;
    if (xp.first / b != yp.first / b) {
      return xp.first / b < yp.first / b;
    }
    int dir = (xp.first / b) % 2;
    return (xp.second < yp.second) ^ dir;
  }
};
void mo_sort(std::vector<pair<pair<int, int>, int> > &queries,
	     int b) {
  sort(queries.begin(), queries.end(), mo_cmp(b));
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  mt19937 mt;
  mt.seed(0xe869120);
  int n, q;
  cin >> n >> q;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  VI ks(q);
  vector<pair<pair<int, int>, int> > queries(q);
  REP(i, 0, q) {
    int l, r;
    cin >> l >> r >> ks[i];
    l--; // [l, r)
    queries[i] = make_pair(PI(l, r), i);
  }
  mo_sort(queries, 550);
  VI freq(n + 1, 0);
  VI ans(q);
  int cl = 0;
  int cr = 0;
  REP(interrapax, 0, q) {
    pair<PI, int> que = queries[interrapax];
    int idx = que.second;
    int l = que.first.first;
    int r = que.first.second;
    int k = ks[idx];
    int thr = (r - l) / k + 1;
    // update freq
    while (cl < l) {
      freq[a[cl]] -= 1;
      cl++;
    }
    while (cl > l) {
      cl--;
      freq[a[cl]] += 1;
    }
    while (cr < r) {
      freq[a[cr]] += 1;
      cr++;
    }
    while (cr > r) {
      cr--;
      freq[a[cr]] -= 1;
    }
    int mi = N;
    int trial = 150;
    if (r - l > trial) {
      REP(puella, 0, trial) {
	int idx = l + (mt() % (r - l));
	if (mi > a[idx]) {
	  int res = freq[a[idx]];
	  if (res >= thr) {
	    mi = a[idx];
	  }
	}
      }
    } else {
      REP(j, l, r) {
	if (mi > a[j]) {
	  int res = freq[a[j]];
	  if (res >= thr) {
	    mi = a[j];
	  }
	}
      }
    }
    if (mi == N) {
      ans[idx] = -1;
    } else {
      ans[idx] = mi;
    }
  }
  REP(i, 0, q) {
    cout << ans[i] << "\n";
  }
}

#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

const int N = 30100;

const int m = 30001;

const int W = 10;

int p[N];

map<int,int> dp[N];
int dp2[N][W];

const int DEBUG = 1;

int n,d;

int ma = 0;


void update(int i, int v) {
	//if(DEBUG) cout << "v=" << v << endl;
	REP(w, v - 1, v + 2) {
		if (w <= 0) {
			continue;
		}
		if(i + w >= m) {
			continue;
		}
		int cur = v >= W ? dp[i][v] : dp2[i][v];
		if(w >= W) {
			dp[i + w][w] = cur + p[i + w];
			ma = max(ma, dp[i + w][w]);
		} else {
			dp2[i + w][w] = cur + p[i + w];
			ma = max(ma, dp2[i + w][w]);
		}
	}

}

int main(void){
	cin >> n >>d;
	REP(i,0,m) {
		p[i]=0;
	}
	REP(i,0,n) {
		int q;
		cin>>q;
		p[q]++;
	}
	map<int,int> init;
	init[d] = 0;
	dp[0] = init;
	REP(i,0,m) {
		if(DEBUG && i % 500 == 0) {
			cout << "i = " << i << endl;
		}
		REP(v, 0, W) {
			update(i, v);
		}
		for(map<int,int>::iterator it = dp[i].begin(); it != dp[i].end(); ++it) {
			int v = it->first;
			//if(DEBUG) cout << "v=" << v << endl;
			update(i, v);
		}

	}
	cout << ma << endl;
}

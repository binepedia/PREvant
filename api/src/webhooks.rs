/*-
 * ========================LICENSE_START=================================
 * PREvant REST API
 * %%
 * Copyright (C) 2018 - 2019 aixigo AG
 * %%
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 * =========================LICENSE_END==================================
 */

use crate::apps::delete_app;
use crate::models::request_info::RequestInfo;
use crate::models::service::Service;
use crate::models::web_hook_info::WebHookInfo;
use crate::models::AppName;
use crate::services::apps_service::AppsService;
use http_api_problem::HttpApiProblem;
use rocket::State;
use rocket_contrib::json::Json;
use std::str::FromStr;

#[post("/webhooks", format = "application/json", data = "<web_hook_info>")]
pub fn webhooks(
    apps_service: State<AppsService>,
    web_hook_info: WebHookInfo,
    request_info: RequestInfo,
) -> Result<Json<Vec<Service>>, HttpApiProblem> {
    info!(
        "Deleting app {:?} through web hook {:?} with event {:?}",
        web_hook_info.get_app_name(),
        web_hook_info.get_title(),
        web_hook_info.get_event_key()
    );

    delete_app(
        AppName::from_str(&web_hook_info.get_app_name()),
        apps_service,
        request_info,
    )
}

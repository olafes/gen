use std::ffi::CString;
use ash::{self, vk, version::{EntryV1_0, DeviceV1_1, EntryV1_1, InstanceV1_1, InstanceV1_0, DeviceV1_0, DeviceV1_2, EntryV1_2, InstanceV1_2}};

// use ash::Entry;

unsafe extern "system" fn vulkan_debug_utils_callback(
  message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
  message_type: vk::DebugUtilsMessageTypeFlagsEXT,
  p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
  _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
  let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
  let severity = format!("{:?}", message_severity).to_lowercase();
  let ty = format!("{:?}", message_type).to_lowercase();
  println!("[Debug][{}][{}] {:?}", severity, ty, message);
  vk::FALSE
}

fn main() {
  let entry = ash::Entry::new().unwrap();

  let application_name = CString::new("gen").unwrap();
  let application_version = ash::vk::make_version(0, 0, 1);
  let api_version = ash::vk::make_version(1, 2, 154);
  let enabled_layers = [CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
  let enabled_extensions: [CString; 0] = [];

  let mut debugcreateinfo = ash::vk::DebugUtilsMessengerCreateInfoEXT::builder()
  .message_severity(
    vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
    // | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
    // | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
    | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
  )
  .message_type(
    vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
  )
  .pfn_user_callback(Some(vulkan_debug_utils_callback));
  let application_info = vk::ApplicationInfo::builder()
    .application_name(&application_name)
    .application_version(application_version)
    .api_version(api_version);
    let raw_enabled_layers: Vec<*const i8> = enabled_layers
    .iter()
    .map(|layer_name| layer_name.as_ptr())
    .collect();
  let raw_enabled_extensions: Vec<*const i8> = enabled_extensions
    .iter()
    .map(|extension_name| extension_name.as_ptr())
    .collect();
  let instance_create_info = vk::InstanceCreateInfo::builder()
    .push_next(&mut debugcreateinfo)
    .application_info(&application_info)
    .enabled_layer_names(&raw_enabled_layers)
    .enabled_extension_names(&raw_enabled_extensions);
  let instance = unsafe { entry.create_instance(&instance_create_info, None).unwrap() };

  let mut physical_devices = unsafe { instance.enumerate_physical_devices().unwrap() };
  
  let physical_device = physical_devices.iter().find(|&&x| {
    let physical_device_properties = unsafe { instance.get_physical_device_properties(x) };
    
    physical_device_properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU 
    // || physical_device_properties.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU
    // || physical_device_properties.device_type == vk::PhysicalDeviceType::VIRTUAL_GPU
  }).unwrap();
  
  let physical_device_properties = unsafe { instance.get_physical_device_properties(*physical_device) };

  let queue_families = unsafe { instance.get_physical_device_queue_family_properties(*physical_device) };
  let memory_properties = unsafe { instance.get_physical_device_memory_properties(*physical_device) };
  
  let graphics_queue_create_info = vk::DeviceQueueCreateInfo::builder()
    .queue_family_index(0) // naprawić to bo jest hardcoded ;_;
    .queue_priorities(&[0.5]);
  let queue_create_infos = vec![graphics_queue_create_info.build()];
  let device_create_info = vk::DeviceCreateInfo::builder()
    .queue_create_infos(&queue_create_infos);
  let device = unsafe { instance.create_device(*physical_device, &device_create_info, None).unwrap() };

  let graphics_queue = unsafe { device.get_device_queue(0, 0) }; // tutaj też hardcoded
  
  // println!("{:?}", physical_device_properties);
  println!("{:#?}", queue_families);
}
